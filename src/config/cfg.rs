//! The configuration struct and related items

use super::{
    parse::{expand_tilde, file_to_string},
    ConfigError,
};
use crate::cli::CliOpts;
use dirs::config_dir;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use structopt::clap::crate_name;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    // path to the HomeBank transactions file
    path: PathBuf,
}

impl Config {
    /// Create a new `Config`
    pub fn new(path: &Path) -> Self {
        Config {
            path: path.to_path_buf(),
        }
    }

    // Retrieve the path to the HomeBank XHB file
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl TryFrom<CliOpts> for Config {
    type Error = ConfigError;

    fn try_from(opts: CliOpts) -> Result<Self, Self::Error> {
        // check that the config file exists
        if !opts.path.exists() {
            return Err(ConfigError::DoesNotExist(opts.path));
        } else if !opts.path.is_file() {
            // check that the config is a file
            return Err(ConfigError::NotAFile(opts.path));
        } else {
            // read the file and parse its contents
            let file_contents = match file_to_string(&opts.path) {
                Ok(s) => s,
                Err(_) => return Err(ConfigError::ParseError(opts.path)),
            };

            // try to deserialize from its contents via toml
            Config::try_from(file_contents.as_str())
        }
    }
}

impl TryFrom<&str> for Config {
    type Error = ConfigError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut cfg: Config = match toml::from_str(s) {
            Ok(cfg) => cfg,
            Err(_) => return Err(ConfigError::MissingHomeBankPath),
        };

        // if the path is tilded, fix it
        if let Some(d) = expand_tilde(cfg.path()) {
            cfg.path = d;
        }

        // check that the HomeBank XHB file is a file
        if !cfg.path().is_file() {
            return Err(ConfigError::HomeBankFileNotAFile(cfg.path().to_path_buf()));
        }

        // check that the HomeBank XHB file is absolute
        if cfg.path().is_relative() {
            return Err(ConfigError::HomeBankFileIsRelative(
                cfg.path().to_path_buf(),
            ));
        }

        Ok(cfg)
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
    #[cfg(target_os = "linux")]
    fn new_absolute_paths_stay_absolute() {
        let input = Path::new("/usr/bin/cat");
        let expected = Config {
            path: PathBuf::from("/usr/bin/cat"),
        };

        check_new(input, expected);
    }

    #[test]
    fn new_existing() {
        let input = Path::new("Cargo.toml");
        let expected = Config {
            path: PathBuf::from("Cargo.toml"),
        };

        check_new(input, expected);
    }

    #[track_caller]
    fn check_try_from_cli(input: CliOpts, expected: Config) {
        let observed = Config::try_from(input).unwrap();

        assert_eq!(expected, observed);
    }

    #[test]
    #[should_panic]
    fn try_from_directory_config() {
        let cli_opts = CliOpts {
            path: PathBuf::from("./src"),
            cmd: None,
        };
        let expected = Config::new(Path::new("path"));

        check_try_from_cli(cli_opts, expected);
    }

    #[test]
    #[should_panic]
    fn try_from_nonexistent_config() {
        let cli_opts = CliOpts {
            path: PathBuf::from("path/to/nonexistent/directory/file.toml"),
            cmd: None,
        };
        let expected = Config::new(Path::new(""));

        check_try_from_cli(cli_opts, expected)
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn try_from_existing_config_absolute_existing_xhb() {
        let input = CliOpts {
            path: PathBuf::from("tests/absolute_existing_linux.toml"),
        };
        let expected = Config {
            path: PathBuf::from("/usr/bin/cat"),
        };

        check_try_from_cli(input, expected);
    }

    #[test]
    #[cfg(target_os = "linux")]
    #[should_panic]
    fn try_from_existing_config_relative_existing_xhb() {
        let input = CliOpts {
            path: PathBuf::from("tests/relative_existing_linux.toml"),
        };
        let expected = Config {
            path: PathBuf::from("/usr/bin/cat"),
        };

        check_try_from_cli(input, expected);
    }

    #[test]
    #[cfg(target_os = "linux")]
    #[should_panic]
    fn try_from_existing_config_absolute_missing_xhb() {
        let input = CliOpts {
            path: PathBuf::from("tests/absolute_missing_linux.toml"),
        };
        let expected = Config {
            path: PathBuf::from("/usr/bin/cat"),
        };

        check_try_from_cli(input, expected);
    }

    #[track_caller]
    fn check_try_from_toml(input: &str, expected: Config) {
        let observed = Config::try_from(input).unwrap();

        assert_eq!(expected, observed);
    }

    #[test]
    #[should_panic]
    fn try_from_str_without_path() {
        let input = "";
        let expected = Config::new(Path::new(""));

        check_try_from_toml(input, expected);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn try_from_str_with_path() {
        let input = "path = '/usr/bin/cat'";
        let expected = Config::new(Path::new("/usr/bin/cat"));

        check_try_from_toml(&input, expected);
    }
}
