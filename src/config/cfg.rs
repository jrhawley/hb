//! The configuration struct and related items

use super::ConfigError;
use crate::cli::CliOpts;
use dirs::config_dir;
use std::path::{Path, PathBuf};
use structopt::clap::crate_name;

#[derive(Debug, PartialEq)]
pub struct Config {
    // path to the configuration TOML file
    path: PathBuf,
}

impl Config {
    /// Create a new `Config`
    pub fn new(path: &Path) -> Self {
        Config {
            path: path.to_path_buf(),
        }
    }
}

impl TryFrom<CliOpts> for Config {
    type Error = ConfigError;

    fn try_from(opts: CliOpts) -> Result<Self, Self::Error> {
        // check that the config file exists
        if !opts.path.exists() {
            return Err(ConfigError::DoesNotExist(opts.path));
        } else if !opts.path.is_file() {
            return Err(ConfigError::NotAFile(opts.path));
        } else {
            Ok(Self::new(&opts.path))
        }
    }
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

    #[track_caller]
    fn check_new(input: &Path, expected: Config) {
        let observed = Config::new(input);

        assert_eq!(expected, observed);
    }

    #[test]
    fn new() {
        let input = Path::new("Cargo.toml");
        let expected = Config {
            path: PathBuf::from("Cargo.toml"),
        };

        check_new(input, expected);
    }

    #[test]
    #[should_panic]
    fn try_from_directory_config() {
        let cli_opts = CliOpts {
            path: PathBuf::from("./src"),
        };

        let _observed = Config::try_from(cli_opts).unwrap();
    }

    #[test]
    #[should_panic]
    fn try_from_nonexistent_config() {
        let cli_opts = CliOpts {
            path: PathBuf::from("path/to/nonexistent/directory/file.toml"),
        };

        let _observed = Config::try_from(cli_opts).unwrap();
    }

    #[track_caller]
    fn check_try_from(input: CliOpts, expected: Config) {
        let observed = Config::try_from(input).unwrap();

        assert_eq!(expected, observed);
    }

    #[test]
    fn try_from_existing_config() {
        let input = CliOpts {
            path: PathBuf::from("Cargo.toml"),
        };
        let expected = Config {
            path: PathBuf::from("Cargo.toml"),
        };

        check_try_from(input, expected);
    }
}
