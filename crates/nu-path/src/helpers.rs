#[cfg(windows)]
use omnipath::WinPathExt;
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
    dirs_next::home_dir()
}

#[cfg(target_os = "macos")]
pub fn config_dir() -> Option<PathBuf> {
    // dirs_next::config_dir returns the path `/Users/<user>/Library/Application Support` on macos.
    // If the user has defined `XDG_CONFIG_HOME` on macos then they expect nushell to act like
    // other *nix systems. This could be defined in a plist file under `$HOME/Library/LaunchAgents`
    // or from the root shell if launched from another shell.
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .ok()
        .or(dirs_next::config_dir())
}

#[cfg(not(target_os = "macos"))]
pub fn config_dir() -> Option<PathBuf> {
    dirs_next::config_dir()
}

#[cfg(windows)]
pub fn canonicalize(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    path.canonicalize()?.to_winuser_path()
}
#[cfg(not(windows))]
pub fn canonicalize(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    path.canonicalize()
}

#[cfg(windows)]
pub fn simiplified(path: &std::path::Path) -> PathBuf {
    path.to_winuser_path()
        .unwrap_or_else(|_| path.to_path_buf())
}
#[cfg(not(windows))]
pub fn simiplified(path: &std::path::Path) -> PathBuf {
    path.to_path_buf()
}
