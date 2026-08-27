use clap::{Parser, ValueEnum};
use file_change_space_check::{
    ConflictPolicy, Manifest, PlanOptions, SpaceVerdict, SparseMode, plan,
};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

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
    long_about = "Scan source and destination metadata, apply an explicit conflict policy, and emit a deterministic copy manifest. fcsc never copies, deletes, renames, or changes permissions."
)]
struct Cli {
    /// File or directory to scan
    source: PathBuf,
    /// Existing destination directory, or a path below one
    destination: PathBuf,
    /// What to plan when a destination-relative path already exists
    #[arg(long, value_enum)]
    policy: PolicyArg,
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
    #[arg(long)]
    no_space_check: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let options = PlanOptions {
        source: cli.source,
        destination: cli.destination,
        policy: match cli.policy {
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
    println!("\nRead-only plan. No files were changed.");
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
