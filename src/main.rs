use clap::{Parser, ValueEnum, error::ErrorKind};
use file_change_space_check::{
    ConflictPolicy, Manifest, PlanOptions, SpaceVerdict, SparseMode, plan,
};
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum PolicyArg {
    Overwrite,
    Skip,
    KeepBoth,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum SparseArg {
    Auto,
    Preserve,
    Expand,
}

#[derive(Debug, Parser)]
#[command(
    name = "fcsc",
    version,
    about = "Preflight disk space and conflicts without touching your files",
    long_about = "Scan source and destination metadata, apply an explicit conflict policy, and emit a path-sorted copy manifest. Planning never copies, deletes, renames, or changes permissions in the source or destination.",
    override_usage = "fcsc <SOURCE> <DESTINATION> --policy <POLICY> [OPTIONS]\n       fcsc --demo [--policy <POLICY>] [--sparse <SPARSE>] [--json]"
)]
struct Cli {
    /// Run a bundled sample in a new temporary directory
    #[arg(long, conflicts_with_all = ["source", "destination", "manifest"])]
    demo: bool,
    /// File or directory to scan
    #[arg(required_unless_present = "demo")]
    source: Option<PathBuf>,
    /// Existing destination directory, or a path below one
    #[arg(required_unless_present = "demo")]
    destination: Option<PathBuf>,
    /// What to plan when a destination-relative path already exists
    #[arg(long, value_enum, required_unless_present = "demo")]
    policy: Option<PolicyArg>,
    /// How to budget sparse file allocation
    #[arg(long, value_enum, default_value_t = SparseArg::Auto)]
    sparse: SparseArg,
    /// Print the complete JSON manifest to stdout
    #[arg(long)]
    json: bool,
    /// Also write the JSON manifest to this file
    #[arg(long, value_name = "FILE")]
    manifest: Option<PathBuf>,
    /// Plan without checking destination free space (exit 3)
    #[arg(long, conflicts_with = "demo")]
    no_space_check: bool,
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let exit_code = match error.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => 0,
                _ => 1,
            };
            let _ = error.print();
            return ExitCode::from(exit_code);
        }
    };
    if cli.demo {
        if cli.source.is_some() || cli.destination.is_some() || cli.manifest.is_some() {
            eprintln!("fcsc: --demo uses its own temporary source, destination, and manifest path");
            return ExitCode::from(1);
        }
        return run_demo(
            cli.policy.unwrap_or(PolicyArg::Overwrite),
            cli.sparse,
            cli.json,
        );
    }

    let (source, destination, policy) = match (cli.source, cli.destination, cli.policy) {
        (Some(source), Some(destination), Some(policy)) => (source, destination, policy),
        _ => {
            eprintln!(
                "fcsc: SOURCE, DESTINATION, and --policy are required unless --demo is used\n\
                 Try 'fcsc --help' for usage."
            );
            return ExitCode::from(1);
        }
    };

    let options = PlanOptions {
        source,
        destination,
        policy: match policy {
            PolicyArg::Overwrite => ConflictPolicy::Overwrite,
            PolicyArg::Skip => ConflictPolicy::Skip,
            PolicyArg::KeepBoth => ConflictPolicy::KeepBoth,
        },
        sparse: match cli.sparse {
            SparseArg::Auto => SparseMode::Auto,
            SparseArg::Preserve => SparseMode::Preserve,
            SparseArg::Expand => SparseMode::Expand,
        },
        check_space: !cli.no_space_check,
    };

    let manifest = match plan(&options) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("fcsc: {error}");
            return ExitCode::from(1);
        }
    };

    let json = match serde_json::to_string_pretty(&manifest) {
        Ok(json) => format!("{json}\n"),
        Err(error) => {
            eprintln!("fcsc: could not serialize manifest: {error}");
            return ExitCode::from(1);
        }
    };

    if let Some(path) = cli.manifest
        && let Err(error) = fs::write(&path, &json)
    {
        eprintln!("fcsc: could not write manifest {}: {error}", path.display());
        return ExitCode::from(1);
    }

    if cli.json {
        print!("{json}");
    } else {
        print_human(&manifest);
    }

    match manifest.verdict {
        SpaceVerdict::Sufficient => ExitCode::SUCCESS,
        SpaceVerdict::Insufficient => ExitCode::from(2),
        SpaceVerdict::Unchecked => ExitCode::from(3),
    }
}

fn run_demo(policy: PolicyArg, sparse: SparseArg, json_only: bool) -> ExitCode {
    let root = match create_demo_fixture() {
        Ok(root) => root,
        Err(error) => {
            eprintln!("fcsc: could not create demo sandbox: {error}");
            return ExitCode::from(1);
        }
    };
    let options = PlanOptions {
        source: root.join("source"),
        destination: root.join("destination"),
        policy: match policy {
            PolicyArg::Overwrite => ConflictPolicy::Overwrite,
            PolicyArg::Skip => ConflictPolicy::Skip,
            PolicyArg::KeepBoth => ConflictPolicy::KeepBoth,
        },
        sparse: match sparse {
            SparseArg::Auto => SparseMode::Auto,
            SparseArg::Preserve => SparseMode::Preserve,
            SparseArg::Expand => SparseMode::Expand,
        },
        check_space: false,
    };
    let manifest = match plan(&options) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("fcsc: demo planning failed: {error}");
            return ExitCode::from(1);
        }
    };
    let json = match serde_json::to_string_pretty(&manifest) {
        Ok(value) => format!("{value}\n"),
        Err(error) => {
            eprintln!("fcsc: could not serialize demo manifest: {error}");
            return ExitCode::from(1);
        }
    };
    let manifest_path = root.join("demo-manifest.json");
    if let Err(error) = fs::write(&manifest_path, &json) {
        eprintln!("fcsc: could not write demo manifest: {error}");
        return ExitCode::from(1);
    }

    if json_only {
        print!("{json}");
    } else {
        println!("DEMO — sample data in a temporary sandbox");
        println!("Nothing in your folders is read or changed.\n");
        print_human(&manifest);
        println!("\nSample sandbox  {}", root.display());
        println!("Manifest        {}", manifest_path.display());
        println!("Reset           run fcsc --demo again for a fresh sandbox");
    }
    ExitCode::SUCCESS
}

fn create_demo_fixture() -> std::io::Result<PathBuf> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("fcsc-demo-{}-{nonce}", std::process::id()));
    let source = root.join("source");
    let destination = root.join("destination");
    fs::create_dir_all(source.join("archive"))?;
    fs::create_dir_all(source.join("disk-images"))?;
    fs::create_dir_all(&destination)?;

    fs::write(
        source.join("archive/project-notes.txt"),
        b"Archive index for the North Shore documentary\n",
    )?;
    write_sized_file(&source.join("archive/interview.mov"), 3 * 1024 * 1024, 0x49)?;
    write_sized_file(&source.join("photos.raw"), 2 * 1024 * 1024, 0x50)?;
    write_sized_file(&destination.join("photos.raw"), 1024 * 1024, 0x4f)?;

    let mut sparse = File::create(source.join("disk-images/field-laptop.img"))?;
    sparse.write_all(b"FCSC demo disk image header")?;
    sparse.seek(SeekFrom::Start(16 * 1024 * 1024 - 1))?;
    sparse.write_all(&[0])?;
    Ok(root)
}

fn write_sized_file(path: &Path, bytes: usize, value: u8) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let block = vec![value; 64 * 1024];
    let mut remaining = bytes;
    while remaining > 0 {
        let count = remaining.min(block.len());
        file.write_all(&block[..count])?;
        remaining -= count;
    }
    Ok(())
}

fn print_human(manifest: &Manifest) {
    let status = match manifest.verdict {
        SpaceVerdict::Sufficient => "SAFE TO START",
        SpaceVerdict::Insufficient => "DO NOT START — INSUFFICIENT SPACE",
        SpaceVerdict::Unchecked => "SPACE NOT CHECKED",
    };
    println!("FILE CHANGE SPACE CHECK");
    println!("Status       {status}");
    println!("Source       {}", manifest.source);
    println!("Destination  {}", manifest.destination);
    println!("Policy       {:?}", manifest.policy);
    match manifest.destination_free_bytes {
        Some(bytes) => println!("Free         {}", format_bytes(bytes)),
        None => println!("Free         unknown"),
    }
    println!(
        "Headroom     {} .. {}",
        format_bytes(manifest.summary.required_headroom_bytes_lower),
        format_bytes(manifest.summary.required_headroom_bytes_upper)
    );
    println!(
        "Net change   {} .. {}",
        format_signed(manifest.summary.net_change_bytes_lower),
        format_signed(manifest.summary.net_change_bytes_upper)
    );
    println!(
        "Conflicts    {}  |  actions {}",
        manifest.summary.conflicts,
        manifest.actions.len()
    );
    if manifest.actions.is_empty() {
        println!("\nNo actions: the source is empty or already has no planned entries.");
    } else {
        println!("\nMANIFEST");
        for action in &manifest.actions {
            println!(
                "{:>4}  {:<22} {} -> {}",
                action.sequence,
                format!("{:?}", action.operation).to_uppercase(),
                action.source,
                action.destination
            );
        }
    }
    println!("\nRead-only plan. No source or destination files were changed.");
}

fn format_signed(bytes: i64) -> String {
    if bytes < 0 {
        format!("-{}", format_bytes(bytes.unsigned_abs()))
    } else {
        format!("+{}", format_bytes(bytes as u64))
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.2} {}", UNITS[unit])
}
