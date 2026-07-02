use std::path::Path;

use crate::GitToolingError;

#[cfg(unix)]
pub fn create_symlink(
    _source: &Path,
    link_target: &Path,
    destination: &Path,
) -> Result<(), GitToolingError> {
    use std::os::unix::fs::symlink;

    symlink(link_target, destination)?;
    Ok(())
}

#[cfg(windows)]
pub fn create_symlink(
    source: &Path,
    link_target: &Path,
    destination: &Path,
) -> Result<(), GitToolingError> {
    use std::os::windows::fs::FileTypeExt;
    use std::os::windows::fs::symlink_dir;
    use std::os::windows::fs::symlink_file;

    let metadata = std::fs::symlink_metadata(source)?;
    if metadata.file_type().is_symlink_dir() {
        symlink_dir(link_target, destination)?;
    } else {
        symlink_file(link_target, destination)?;
    }
    Ok(())
}

#[cfg(not(any(unix, windows)))]
compile_error!("codex-git symlink support is only implemented for Unix and Windows");

/// A `git` command whose output is captured (never interactive). On Windows
/// the child is created with CREATE_NO_WINDOW: when codex runs inside a GUI
/// host process (no console), an unflagged console-subsystem child would
/// otherwise open a visible console window (e.g. the session-start git-info
/// collection flashing a terminal).
pub(crate) fn git_command() -> std::process::Command {
    #[allow(unused_mut)]
    let mut command = std::process::Command::new("git");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    command
}

/// Async variant of [`git_command`].
pub(crate) fn git_command_async() -> tokio::process::Command {
    #[allow(unused_mut)]
    let mut command = tokio::process::Command::new("git");
    #[cfg(windows)]
    command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    command
}
