//! Read-only planning engine for `fcsc`.
//!
//! The public surface is intentionally small: construct [`PlanOptions`] and
//! call [`plan`]. Planning reads metadata but never mutates either tree.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ffi::CString;
use std::fs::{self, Metadata};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/// Conflict behavior for a destination path that already exists.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictPolicy {
    Overwrite,
    Skip,
    KeepBoth,
}

/// How sparse source files should be budgeted on the destination.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SparseMode {
    /// Report allocated-to-apparent-size bounds and decide with the upper one.
    Auto,
    /// Assume the eventual copy tool preserves holes.
    Preserve,
    /// Assume every apparent byte is allocated.
    Expand,
}

/// Options for one planning run.
#[derive(Clone, Debug)]
pub struct PlanOptions {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub policy: ConflictPolicy,
    pub sparse: SparseMode,
    pub check_space: bool,
}

/// A stable, machine-readable description of a planned action.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Action {
    pub sequence: usize,
    pub operation: Operation,
    pub kind: EntryKind,
    pub source: String,
    pub destination: String,
    pub conflict: bool,
    pub write_bytes_lower: u64,
    pub write_bytes_upper: u64,
    pub reclaimable_bytes: u64,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Operation {
    Copy,
    Overwrite,
    Skip,
    CreateDirectory,
    ReplaceWithDirectory,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Summary {
    pub files_scanned: u64,
    pub directories_scanned: u64,
    pub symlinks_scanned: u64,
    pub conflicts: u64,
    pub copies: u64,
    pub overwrites: u64,
    pub skipped: u64,
    pub directories_created: u64,
    pub write_bytes_lower: u64,
    pub write_bytes_upper: u64,
    pub reclaimable_bytes: u64,
    pub net_change_bytes_lower: i64,
    pub net_change_bytes_upper: i64,
    pub required_headroom_bytes_lower: u64,
    pub required_headroom_bytes_upper: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpaceVerdict {
    Sufficient,
    Insufficient,
    Unchecked,
}

/// Complete deterministic plan. It intentionally contains no timestamp.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub schema_version: u8,
    pub source: String,
    pub destination: String,
    pub policy: ConflictPolicy,
    pub sparse: SparseMode,
    pub destination_block_size: u64,
    pub destination_free_bytes: Option<u64>,
    pub verdict: SpaceVerdict,
    pub summary: Summary,
    pub actions: Vec<Action>,
    pub notes: Vec<String>,
}

#[derive(Debug)]
pub enum PlanError {
    InvalidInput(String),
    Io { path: PathBuf, source: io::Error },
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(formatter, "{message}"),
            Self::Io { path, source } => {
                write!(formatter, "cannot read {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for PlanError {}

struct Planner {
    policy: ConflictPolicy,
    sparse: SparseMode,
    block_size: u64,
    actions: Vec<Action>,
    summary: Summary,
    reserved: HashSet<PathBuf>,
}

/// Scan source and destination metadata and produce an action manifest.
///
/// A source directory maps its contents into the destination root. A source
/// file maps to `destination/source-file-name`.
///
/// ```no_run
/// use file_change_space_check::{plan, ConflictPolicy, PlanOptions, SparseMode};
/// use std::path::PathBuf;
///
/// let manifest = plan(&PlanOptions {
///     source: PathBuf::from("./camera-roll"),
///     destination: PathBuf::from("/mnt/archive"),
///     policy: ConflictPolicy::Overwrite,
///     sparse: SparseMode::Auto,
///     check_space: true,
/// })?;
/// println!("{:?}", manifest.verdict);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn plan(options: &PlanOptions) -> Result<Manifest, PlanError> {
    let source_input = absolute_lexical(&options.source)?;
    let source_meta = metadata(&source_input)?;
    let source = if source_meta.is_dir() {
        canonical_existing(&source_input)?
    } else {
        source_input
    };
    let source_meta = metadata(&source)?;
    if !(source_meta.is_file() || source_meta.is_dir() || source_meta.file_type().is_symlink()) {
        return Err(PlanError::InvalidInput(format!(
            "source {} is not a regular file, directory, or symlink",
            source.display()
        )));
    }

    let destination = absolute_lexical(&options.destination)?;
    if fs::symlink_metadata(&destination).is_ok() {
        let meta = fs::metadata(&destination).map_err(|source| PlanError::Io {
            path: destination.clone(),
            source,
        })?;
        if !meta.is_dir() {
            return Err(PlanError::InvalidInput(format!(
                "destination {} exists but is not a directory",
                destination.display()
            )));
        }
    }
    let resolved_destination = resolve_destination_ancestry(&destination)?;
    if source_meta.is_dir() {
        if resolved_destination == source {
            return Err(PlanError::InvalidInput(
                "source and destination directories must be different".into(),
            ));
        }
        if resolved_destination.starts_with(&source) {
            return Err(PlanError::InvalidInput(
                "destination cannot be inside the source tree".into(),
            ));
        }
    }

    let space_path = nearest_existing_ancestor(&destination)?;
    let fs_info = match filesystem_info(&space_path) {
        Ok(info) => info,
        Err(_) if !options.check_space => FilesystemInfo {
            block_size: 4096,
            free_bytes: 0,
        },
        Err(error) => return Err(error),
    };
    let mut planner = Planner {
        policy: options.policy,
        sparse: options.sparse,
        block_size: fs_info.block_size.max(1),
        actions: Vec::new(),
        summary: Summary::default(),
        reserved: HashSet::new(),
    };

    let destination_exists = destination.is_dir();
    if !destination_exists {
        planner.push_action(
            Operation::CreateDirectory,
            EntryKind::Directory,
            Path::new("."),
            &destination,
            false,
            (0, 0),
            0,
            "destination root does not exist",
        );
        planner.summary.directories_created += 1;
    }

    if source_meta.is_dir() {
        planner.summary.directories_scanned += 1;
        planner.scan_children(&source, &destination, !destination_exists)?;
    } else {
        let name = source
            .file_name()
            .ok_or_else(|| PlanError::InvalidInput("source must have a file name".into()))?;
        planner.reserve_destination(&destination.join(name));
        planner.scan_entry(
            &source,
            Path::new(name),
            &destination.join(name),
            !destination_exists,
        )?;
    }

    planner.summary.net_change_bytes_lower = signed_difference(
        planner.summary.write_bytes_lower,
        planner.summary.reclaimable_bytes,
    );
    planner.summary.net_change_bytes_upper = signed_difference(
        planner.summary.write_bytes_upper,
        planner.summary.reclaimable_bytes,
    );
    planner.summary.required_headroom_bytes_lower = planner.summary.write_bytes_lower;
    planner.summary.required_headroom_bytes_upper = planner.summary.write_bytes_upper;

    let free = options.check_space.then_some(fs_info.free_bytes);
    let verdict = space_verdict(free, planner.summary.required_headroom_bytes_upper);

    let mut notes = vec![
        "Read-only metadata plan: permissions and the eventual copy tool are not tested.".into(),
        "Headroom budgets all writes before reclaiming overwritten destination bytes.".into(),
    ];
    if options.sparse == SparseMode::Auto {
        notes.push(
            "Sparse allocation is a range; the upper bound assumes holes expand and controls the verdict."
                .into(),
        );
    }

    Ok(Manifest {
        schema_version: 1,
        source: path_string(&source),
        destination: path_string(&destination),
        policy: options.policy,
        sparse: options.sparse,
        destination_block_size: planner.block_size,
        destination_free_bytes: free,
        verdict,
        summary: planner.summary,
        actions: planner.actions,
        notes,
    })
}

impl Planner {
    fn scan_children(
        &mut self,
        source_dir: &Path,
        destination_dir: &Path,
        destination_virtual: bool,
    ) -> Result<(), PlanError> {
        let mut entries = fs::read_dir(source_dir)
            .map_err(|source| PlanError::Io {
                path: source_dir.to_path_buf(),
                source,
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| PlanError::Io {
                path: source_dir.to_path_buf(),
                source,
            })?;
        entries.sort_by_key(|entry| entry.file_name());
        self.reserve_child_destinations(&entries, destination_dir);
        for entry in entries {
            let name = entry.file_name();
            self.scan_entry(
                &entry.path(),
                Path::new(&name),
                &destination_dir.join(&name),
                destination_virtual,
            )?;
        }
        Ok(())
    }

    fn scan_entry(
        &mut self,
        source: &Path,
        relative: &Path,
        destination: &Path,
        destination_virtual: bool,
    ) -> Result<(), PlanError> {
        let source_meta = metadata(source)?;
        let kind = entry_kind(&source_meta);
        match kind {
            EntryKind::File => self.summary.files_scanned += 1,
            EntryKind::Directory => self.summary.directories_scanned += 1,
            EntryKind::Symlink => self.summary.symlinks_scanned += 1,
            EntryKind::Other => {}
        }

        let destination_meta = if destination_virtual {
            None
        } else {
            optional_metadata(destination)?
        };

        if kind == EntryKind::Directory {
            self.scan_directory(source, relative, destination, destination_meta)
        } else {
            self.scan_leaf(
                source,
                relative,
                destination,
                kind,
                &source_meta,
                destination_meta,
            )
        }
    }

    fn scan_directory(
        &mut self,
        source: &Path,
        relative: &Path,
        destination: &Path,
        destination_meta: Option<Metadata>,
    ) -> Result<(), PlanError> {
        match destination_meta {
            None => {
                self.push_action(
                    Operation::CreateDirectory,
                    EntryKind::Directory,
                    relative,
                    destination,
                    false,
                    (0, 0),
                    0,
                    "directory is missing at destination",
                );
                self.summary.directories_created += 1;
                self.scan_children_with_prefix(source, relative, destination, true)
            }
            Some(meta) if meta.is_dir() => {
                self.scan_children_with_prefix(source, relative, destination, false)
            }
            Some(meta) => {
                self.summary.conflicts += 1;
                match self.policy {
                    ConflictPolicy::Skip => {
                        self.push_action(
                            Operation::Skip,
                            EntryKind::Directory,
                            relative,
                            destination,
                            true,
                            (0, 0),
                            0,
                            "type conflict; source subtree is skipped",
                        );
                        self.summary.skipped += 1;
                        Ok(())
                    }
                    ConflictPolicy::Overwrite => {
                        let reclaim = allocated_tree(destination, &meta)?;
                        self.push_action(
                            Operation::ReplaceWithDirectory,
                            EntryKind::Directory,
                            relative,
                            destination,
                            true,
                            (0, 0),
                            reclaim,
                            "replace destination entry with a directory",
                        );
                        self.summary.overwrites += 1;
                        self.add_reclaim(reclaim);
                        self.scan_children_with_prefix(source, relative, destination, true)
                    }
                    ConflictPolicy::KeepBoth => {
                        let alternate = self.keep_both_path(destination);
                        self.push_action(
                            Operation::CreateDirectory,
                            EntryKind::Directory,
                            relative,
                            &alternate,
                            true,
                            (0, 0),
                            0,
                            "type conflict; use a deterministic alternate name",
                        );
                        self.summary.directories_created += 1;
                        self.scan_children_with_prefix(source, relative, &alternate, true)
                    }
                }
            }
        }
    }

    fn scan_children_with_prefix(
        &mut self,
        source_dir: &Path,
        relative_dir: &Path,
        destination_dir: &Path,
        destination_virtual: bool,
    ) -> Result<(), PlanError> {
        let mut entries = fs::read_dir(source_dir)
            .map_err(|source| PlanError::Io {
                path: source_dir.to_path_buf(),
                source,
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| PlanError::Io {
                path: source_dir.to_path_buf(),
                source,
            })?;
        entries.sort_by_key(|entry| entry.file_name());
        self.reserve_child_destinations(&entries, destination_dir);
        for entry in entries {
            let name = entry.file_name();
            self.scan_entry(
                &entry.path(),
                &relative_dir.join(&name),
                &destination_dir.join(&name),
                destination_virtual,
            )?;
        }
        Ok(())
    }

    fn scan_leaf(
        &mut self,
        _source: &Path,
        relative: &Path,
        destination: &Path,
        kind: EntryKind,
        source_meta: &Metadata,
        destination_meta: Option<Metadata>,
    ) -> Result<(), PlanError> {
        if kind == EntryKind::Other {
            self.push_action(
                Operation::Skip,
                kind,
                relative,
                destination,
                destination_meta.is_some(),
                (0, 0),
                0,
                "special filesystem entries are not copy-planned",
            );
            self.summary.skipped += 1;
            return Ok(());
        }
        let bounds = allocation_bounds(source_meta, self.block_size, self.sparse, kind);
        match destination_meta {
            None => {
                self.push_action(
                    Operation::Copy,
                    kind,
                    relative,
                    destination,
                    false,
                    bounds,
                    0,
                    "destination path is available",
                );
                self.summary.copies += 1;
                self.add_write(bounds);
            }
            Some(meta) => {
                self.summary.conflicts += 1;
                match self.policy {
                    ConflictPolicy::Skip => {
                        self.push_action(
                            Operation::Skip,
                            kind,
                            relative,
                            destination,
                            true,
                            (0, 0),
                            0,
                            "destination path exists",
                        );
                        self.summary.skipped += 1;
                    }
                    ConflictPolicy::Overwrite => {
                        let reclaim = allocated_tree(destination, &meta)?;
                        self.push_action(
                            Operation::Overwrite,
                            kind,
                            relative,
                            destination,
                            true,
                            bounds,
                            reclaim,
                            "destination path exists",
                        );
                        self.summary.overwrites += 1;
                        self.add_write(bounds);
                        self.add_reclaim(reclaim);
                    }
                    ConflictPolicy::KeepBoth => {
                        let alternate = self.keep_both_path(destination);
                        self.push_action(
                            Operation::Copy,
                            kind,
                            relative,
                            &alternate,
                            true,
                            bounds,
                            0,
                            "destination path exists; use a deterministic alternate name",
                        );
                        self.summary.copies += 1;
                        self.add_write(bounds);
                    }
                }
            }
        }
        Ok(())
    }

    fn keep_both_path(&self, original: &Path) -> PathBuf {
        let parent = original.parent().unwrap_or_else(|| Path::new(""));
        let name = original.file_name().unwrap_or_default().to_string_lossy();
        let (stem, extension) = match name.rsplit_once('.') {
            Some((stem, extension)) if !stem.is_empty() => {
                (stem.to_string(), format!(".{extension}"))
            }
            _ => (name.into_owned(), String::new()),
        };
        for number in 1u64.. {
            let candidate = parent.join(format!("{stem} (copy {number}){extension}"));
            if !candidate.exists() && !self.reserved.contains(&candidate) {
                return candidate;
            }
        }
        unreachable!()
    }

    fn reserve_child_destinations(&mut self, entries: &[fs::DirEntry], destination_dir: &Path) {
        for entry in entries {
            self.reserve_destination(&destination_dir.join(entry.file_name()));
        }
    }

    fn reserve_destination(&mut self, destination: &Path) {
        self.reserved.insert(destination.to_path_buf());
    }

    #[allow(clippy::too_many_arguments)]
    fn push_action(
        &mut self,
        operation: Operation,
        kind: EntryKind,
        source: &Path,
        destination: &Path,
        conflict: bool,
        write: (u64, u64),
        reclaimable: u64,
        reason: &str,
    ) {
        self.reserve_destination(destination);
        self.actions.push(Action {
            sequence: self.actions.len() + 1,
            operation,
            kind,
            source: path_string(source),
            destination: path_string(destination),
            conflict,
            write_bytes_lower: write.0,
            write_bytes_upper: write.1,
            reclaimable_bytes: reclaimable,
            reason: reason.into(),
        });
    }

    fn add_write(&mut self, bounds: (u64, u64)) {
        self.summary.write_bytes_lower = self.summary.write_bytes_lower.saturating_add(bounds.0);
        self.summary.write_bytes_upper = self.summary.write_bytes_upper.saturating_add(bounds.1);
    }

    fn add_reclaim(&mut self, bytes: u64) {
        self.summary.reclaimable_bytes = self.summary.reclaimable_bytes.saturating_add(bytes);
    }
}

fn entry_kind(metadata: &Metadata) -> EntryKind {
    let file_type = metadata.file_type();
    if file_type.is_file() {
        EntryKind::File
    } else if file_type.is_dir() {
        EntryKind::Directory
    } else if file_type.is_symlink() {
        EntryKind::Symlink
    } else {
        EntryKind::Other
    }
}

fn allocation_bounds(
    metadata: &Metadata,
    block_size: u64,
    sparse: SparseMode,
    kind: EntryKind,
) -> (u64, u64) {
    if kind == EntryKind::Directory || kind == EntryKind::Other {
        return (0, 0);
    }
    let allocated = round_up(metadata.blocks().saturating_mul(512), block_size);
    let expanded = round_up(metadata.len(), block_size);
    match sparse {
        SparseMode::Auto => (allocated.min(expanded), allocated.max(expanded)),
        SparseMode::Preserve => (allocated, allocated),
        SparseMode::Expand => (expanded, expanded),
    }
}

fn allocated_tree(path: &Path, entry_metadata: &Metadata) -> Result<u64, PlanError> {
    if !entry_metadata.is_dir() {
        return Ok(entry_metadata.blocks().saturating_mul(512));
    }
    let mut total = 0u64;
    let entries = fs::read_dir(path)
        .map_err(|source| PlanError::Io {
            path: path.to_path_buf(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| PlanError::Io {
            path: path.to_path_buf(),
            source,
        })?;
    for entry in entries {
        let child = entry.path();
        let meta = metadata(&child)?;
        total = total.saturating_add(allocated_tree(&child, &meta)?);
    }
    Ok(total)
}

fn round_up(value: u64, block: u64) -> u64 {
    if value == 0 {
        0
    } else {
        value.saturating_add(block - 1) / block * block
    }
}

fn signed_difference(left: u64, right: u64) -> i64 {
    let difference = i128::from(left) - i128::from(right);
    difference.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

fn space_verdict(free: Option<u64>, required_upper: u64) -> SpaceVerdict {
    match free {
        Some(bytes) if bytes >= required_upper => SpaceVerdict::Sufficient,
        Some(_) => SpaceVerdict::Insufficient,
        None => SpaceVerdict::Unchecked,
    }
}

fn metadata(path: &Path) -> Result<Metadata, PlanError> {
    fs::symlink_metadata(path).map_err(|source| PlanError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn optional_metadata(path: &Path) -> Result<Option<Metadata>, PlanError> {
    match fs::symlink_metadata(path) {
        Ok(meta) => Ok(Some(meta)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(PlanError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn canonical_existing(path: &Path) -> Result<PathBuf, PlanError> {
    fs::canonicalize(path).map_err(|source| PlanError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn absolute_lexical(path: &Path) -> Result<PathBuf, PlanError> {
    let combined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|source| PlanError::Io {
                path: PathBuf::from("."),
                source,
            })?
            .join(path)
    };
    let mut clean = PathBuf::new();
    for component in combined.components() {
        use std::path::Component;
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                clean.pop();
            }
            other => clean.push(other.as_os_str()),
        }
    }
    Ok(clean)
}

fn nearest_existing_ancestor(path: &Path) -> Result<PathBuf, PlanError> {
    let mut candidate = path.to_path_buf();
    loop {
        match fs::metadata(&candidate) {
            Ok(meta) if meta.is_dir() => return Ok(candidate),
            Ok(_) => {
                candidate.pop();
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                if !candidate.pop() {
                    return Err(PlanError::InvalidInput(format!(
                        "destination {} has no existing ancestor",
                        path.display()
                    )));
                }
            }
            Err(source) => {
                return Err(PlanError::Io {
                    path: candidate,
                    source,
                });
            }
        }
    }
}

/// Resolve every existing destination component while leaving a missing tail
/// lexical. This catches paths that enter the source through a symlink before
/// the requested destination exists.
fn resolve_destination_ancestry(destination: &Path) -> Result<PathBuf, PlanError> {
    let ancestor = nearest_existing_ancestor(destination)?;
    let resolved_ancestor = canonical_existing(&ancestor)?;
    let tail = destination.strip_prefix(&ancestor).map_err(|_| {
        PlanError::InvalidInput(format!(
            "destination {} has an invalid existing ancestor",
            destination.display()
        ))
    })?;
    Ok(resolved_ancestor.join(tail))
}

struct FilesystemInfo {
    block_size: u64,
    free_bytes: u64,
}

fn filesystem_info(path: &Path) -> Result<FilesystemInfo, PlanError> {
    let path_bytes = path.as_os_str().as_bytes();
    let c_path = CString::new(path_bytes).map_err(|_| {
        PlanError::InvalidInput(format!("path contains a null byte: {}", path.display()))
    })?;
    let mut stats = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    // SAFETY: `c_path` is NUL-terminated and `stats` points to writable memory.
    let result = unsafe { libc::statvfs(c_path.as_ptr(), stats.as_mut_ptr()) };
    if result != 0 {
        return Err(PlanError::Io {
            path: path.to_path_buf(),
            source: io::Error::last_os_error(),
        });
    }
    // SAFETY: statvfs returned success and initialized the structure.
    let stats = unsafe { stats.assume_init() };
    let block_size = stats.f_bsize;
    let fragment_size = stats.f_frsize.max(1).min(block_size.max(1));
    Ok(FilesystemInfo {
        block_size: fragment_size,
        free_bytes: stats.f_bavail.saturating_mul(fragment_size),
    })
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!("fcsc-{}-{nonce}", std::process::id()));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn options(root: &Path, policy: ConflictPolicy) -> PlanOptions {
        PlanOptions {
            source: root.join("source"),
            destination: root.join("destination"),
            policy,
            sparse: SparseMode::Auto,
            check_space: true,
        }
    }

    #[test]
    fn plans_empty_directory_without_writes() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("source")).unwrap();
        fs::create_dir(fixture.0.join("destination")).unwrap();
        let manifest = plan(&options(&fixture.0, ConflictPolicy::Overwrite)).unwrap();
        assert!(manifest.actions.is_empty());
        assert_eq!(manifest.summary.write_bytes_upper, 0);
        assert_eq!(manifest.verdict, SpaceVerdict::Sufficient);
    }

    #[test]
    fn conflict_policies_change_actions_and_bytes() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("source")).unwrap();
        fs::create_dir(fixture.0.join("destination")).unwrap();
        fs::write(fixture.0.join("source/report.txt"), b"new report").unwrap();
        fs::write(fixture.0.join("destination/report.txt"), b"old").unwrap();

        let skip = plan(&options(&fixture.0, ConflictPolicy::Skip)).unwrap();
        assert_eq!(skip.actions[0].operation, Operation::Skip);
        assert_eq!(skip.summary.write_bytes_upper, 0);

        let overwrite = plan(&options(&fixture.0, ConflictPolicy::Overwrite)).unwrap();
        assert_eq!(overwrite.actions[0].operation, Operation::Overwrite);
        assert!(overwrite.summary.write_bytes_upper > 0);
        assert!(overwrite.summary.reclaimable_bytes > 0);

        let keep = plan(&options(&fixture.0, ConflictPolicy::KeepBoth)).unwrap();
        assert!(keep.actions[0].destination.ends_with("report (copy 1).txt"));
        assert_eq!(keep.actions[0].operation, Operation::Copy);
    }

    #[test]
    fn manifest_is_deterministic_and_sorted() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("source")).unwrap();
        fs::create_dir(fixture.0.join("destination")).unwrap();
        fs::write(fixture.0.join("source/z.txt"), b"z").unwrap();
        fs::write(fixture.0.join("source/a.txt"), b"a").unwrap();
        let mut settings = options(&fixture.0, ConflictPolicy::Overwrite);
        settings.check_space = false;
        let first = serde_json::to_string(&plan(&settings).unwrap()).unwrap();
        let second = serde_json::to_string(&plan(&settings).unwrap()).unwrap();
        assert_eq!(first, second);
        assert!(first.find("a.txt").unwrap() < first.find("z.txt").unwrap());
    }

    #[test]
    fn sparse_auto_exposes_allocation_range() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("source")).unwrap();
        fs::create_dir(fixture.0.join("destination")).unwrap();
        let file = File::create(fixture.0.join("source/sparse.bin")).unwrap();
        file.set_len(64 * 1024 * 1024).unwrap();
        let manifest = plan(&options(&fixture.0, ConflictPolicy::Overwrite)).unwrap();
        assert!(manifest.summary.write_bytes_upper >= 64 * 1024 * 1024);
        assert!(manifest.summary.write_bytes_lower < manifest.summary.write_bytes_upper);
    }

    #[test]
    fn keep_both_avoids_existing_suffixes() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("source")).unwrap();
        fs::create_dir(fixture.0.join("destination")).unwrap();
        for path in [
            "source/photo.jpg",
            "destination/photo.jpg",
            "destination/photo (copy 1).jpg",
        ] {
            let mut file = File::create(fixture.0.join(path)).unwrap();
            file.write_all(b"pixels").unwrap();
        }
        let manifest = plan(&options(&fixture.0, ConflictPolicy::KeepBoth)).unwrap();
        assert!(
            manifest.actions[0]
                .destination
                .ends_with("photo (copy 2).jpg")
        );
    }

    #[test]
    fn keep_both_preserves_source_entries_with_generated_alternate_names() {
        let fixture = Fixture::new();
        let source = fixture.0.join("source");
        let destination = fixture.0.join("destination");
        fs::create_dir(&source).unwrap();
        fs::create_dir(&destination).unwrap();

        fs::write(source.join("photo.jpg"), b"original photo").unwrap();
        fs::write(source.join("photo (copy 1).jpg"), b"named copy").unwrap();
        fs::write(destination.join("photo.jpg"), b"destination photo").unwrap();

        fs::create_dir(source.join("album")).unwrap();
        fs::write(source.join("album/from-original.txt"), b"original album").unwrap();
        fs::create_dir(source.join("album (copy 1)")).unwrap();
        fs::write(source.join("album (copy 1)/named-copy.txt"), b"named album").unwrap();
        fs::write(destination.join("album"), b"destination file").unwrap();

        let manifest = plan(&options(&fixture.0, ConflictPolicy::KeepBoth)).unwrap();
        let destinations: HashSet<_> = manifest
            .actions
            .iter()
            .map(|action| action.destination.as_str())
            .collect();
        assert_eq!(destinations.len(), manifest.actions.len());

        let photo_destination = |source: &str| {
            manifest
                .actions
                .iter()
                .find(|action| action.source == source)
                .expect("each source photo has a planned action")
                .destination
                .as_str()
        };
        assert!(photo_destination("photo (copy 1).jpg").ends_with("photo (copy 1).jpg"));
        assert!(photo_destination("photo.jpg").ends_with("photo (copy 2).jpg"));

        let album_destination = |source: &str| {
            manifest
                .actions
                .iter()
                .find(|action| action.source == source && action.kind == EntryKind::Directory)
                .expect("each source album has a planned directory action")
                .destination
                .as_str()
        };
        assert!(album_destination("album (copy 1)").ends_with("album (copy 1)"));
        assert!(album_destination("album").ends_with("album (copy 2)"));
    }

    #[test]
    fn estimates_match_actual_dense_copy_for_every_policy() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("source")).unwrap();
        fs::write(fixture.0.join("source/new.bin"), vec![0xA5; 1024 * 1024]).unwrap();
        fs::write(
            fixture.0.join("source/conflict.bin"),
            vec![0x5A; 2 * 1024 * 1024],
        )
        .unwrap();

        for policy in [
            ConflictPolicy::Overwrite,
            ConflictPolicy::Skip,
            ConflictPolicy::KeepBoth,
        ] {
            let destination = fixture.0.join(format!("destination-{policy:?}"));
            fs::create_dir(&destination).unwrap();
            fs::write(destination.join("conflict.bin"), vec![0x11; 512 * 1024]).unwrap();
            let before = allocated_regular_files(&destination);
            let manifest = plan(&PlanOptions {
                source: fixture.0.join("source"),
                destination: destination.clone(),
                policy,
                sparse: SparseMode::Expand,
                check_space: true,
            })
            .unwrap();

            fs::copy(
                fixture.0.join("source/new.bin"),
                destination.join("new.bin"),
            )
            .unwrap();
            match policy {
                ConflictPolicy::Overwrite => {
                    fs::copy(
                        fixture.0.join("source/conflict.bin"),
                        destination.join("conflict.bin"),
                    )
                    .unwrap();
                }
                ConflictPolicy::Skip => {}
                ConflictPolicy::KeepBoth => {
                    fs::copy(
                        fixture.0.join("source/conflict.bin"),
                        destination.join("conflict (copy 1).bin"),
                    )
                    .unwrap();
                }
            }
            let actual = signed_difference(allocated_regular_files(&destination), before);
            let estimated = manifest.summary.net_change_bytes_upper;
            let tolerance = ((actual.unsigned_abs() as f64) * 0.02).ceil() as i64;
            assert!(
                (estimated - actual).abs() <= tolerance,
                "{policy:?}: estimated {estimated}, actual {actual}, tolerance {tolerance}"
            );
        }
    }

    #[test]
    fn upper_bound_controls_space_verdict() {
        assert_eq!(
            space_verdict(Some(10_000), 10_000),
            SpaceVerdict::Sufficient
        );
        assert_eq!(
            space_verdict(Some(9_999), 10_000),
            SpaceVerdict::Insufficient
        );
        assert_eq!(space_verdict(None, 0), SpaceVerdict::Unchecked);
    }

    #[test]
    fn rejects_destination_inside_source_through_symlinked_ancestor() {
        use std::os::unix::fs::symlink;

        let fixture = Fixture::new();
        let source = fixture.0.join("source");
        fs::create_dir(&source).unwrap();
        fs::write(source.join("item.bin"), b"payload").unwrap();
        symlink(&source, fixture.0.join("source-alias")).unwrap();

        let error = plan(&PlanOptions {
            source,
            destination: fixture.0.join("source-alias/new-subdir"),
            policy: ConflictPolicy::Overwrite,
            sparse: SparseMode::Auto,
            check_space: false,
        })
        .unwrap_err();

        assert!(matches!(error, PlanError::InvalidInput(_)));
        assert_eq!(
            error.to_string(),
            "destination cannot be inside the source tree"
        );
    }

    fn allocated_regular_files(directory: &Path) -> u64 {
        fs::read_dir(directory)
            .unwrap()
            .map(|entry| entry.unwrap())
            .filter_map(|entry| {
                let metadata = entry.metadata().unwrap();
                metadata.is_file().then(|| metadata.blocks() * 512)
            })
            .sum()
    }
}
