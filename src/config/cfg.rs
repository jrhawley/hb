//! The configuration struct and related items

use std::path::PathBuf;

use dirs::config_dir;
use structopt::clap::crate_name;

pub struct Config {
    // path to the configuration TOML file
    path: PathBuf,
}

/// The default folder for the application's configuration
fn default_cfg_dir() -> PathBuf {
    let cfg_dir = match config_dir() {
        Some(d) => d,
        None => PathBuf::from("~/.config"),
    };

    cfg_dir.join(crate_name!())
}

/// The default configuration file
pub fn default_cfg_file() -> PathBuf {
    default_cfg_dir().join("config.toml")
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs::home_dir;

    #[test]
    #[cfg(target_os = "linux")]
    fn default_linux_config_dir() {
        let expected = home_dir().unwrap().join(".config/hb/");
        let observed = default_cfg_dir();

        assert_eq!(expected, observed);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn default_windows_config_dir() {
        use std::path::Path;

        let expected = home_dir().unwrap().join("AppData/Roaming/hb/");
        let observed = default_cfg_dir();

        assert_eq!(expected, observed);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn default_macos_config_dir() {
        use std::path::Path;

        let expected = home_dir().unwrap().join("Library/Application Support/hb/");
        let observed = default_cfg_dir();

        assert_eq!(expected, observed);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn default_linux_config_file() {
        let expected = home_dir().unwrap().join(".config/hb/config.toml");
        let observed = default_cfg_file();

        assert_eq!(expected, observed);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn default_windows_config_file() {
        use std::path::Path;

        let expected = home_dir().unwrap().join("AppData/Roaming/hb/config.toml");
        let observed = default_cfg_file();

        assert_eq!(expected, observed);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn default_macos_config_file() {
        use std::path::Path;

        let expected = home_dir()
            .unwrap()
            .join("Library/Application Support/hb/config.toml");
        let observed = default_cfg_file();

        assert_eq!(expected, observed);
    }
}
