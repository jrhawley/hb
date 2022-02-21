//! Utilities for working with TOML files.

use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn file_to_string(path: &Path) -> io::Result<String> {
    // open the file for reading
    let mut file = File::open(path)?;

    // read the file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
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
}
