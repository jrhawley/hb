//! Utilities for working with TOML files.

use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use dirs::home_dir;

/// Read the contents of a file into a string, if possible
pub fn file_to_string(path: &Path) -> io::Result<String> {
    // open the file for reading
    let mut file = File::open(path)?;

    // read the file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

/// Replace the `~` character in any path with the home directory.
/// See <https://stackoverflow.com/a/54306906/7416009>
pub fn expand_tilde<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let p = path.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return home_dir();
    }
    home_dir().map(|mut h| {
        if h == Path::new("/") {
            // base case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[track_caller]
    fn check_file_to_string(input: &Path, expected: &str) {
        let observed = file_to_string(input).unwrap();

        assert_eq!(expected, observed);
    }

    #[test]
    fn read_empty() {
        let input = Path::new("tests/empty_file.toml");
        let expected = "";

        check_file_to_string(input, expected);
    }

    #[test]
    #[should_panic]
    fn read_nonexistent() {
        let input = Path::new("tests/there/is/no/file/found/here/empty_file.toml");
        let expected = "";

        check_file_to_string(input, expected);
    }

    #[track_caller]
    fn check_expand_tilde(input: &Path, expected: Option<PathBuf>) {
        let observed = expand_tilde(input);

        assert_eq!(expected, observed);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn non_tilde_is_unchanged() {
        let input = Path::new("/usr/bin/cat");
        let expected = Some(PathBuf::from("/usr/bin/cat"));

        check_expand_tilde(input, expected);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn tilded_is_changed() {
        let input = Path::new("~");
        let expected = home_dir();

        check_expand_tilde(input, expected);
    }
}
