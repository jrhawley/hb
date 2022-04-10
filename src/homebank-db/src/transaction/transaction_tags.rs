//! Helper functions to handle the processing of transaction tags

const TAG_SEPARATOR: char = ' ';

pub(crate) fn split_tags(s: &str) -> Vec<String> {
    s.split(TAG_SEPARATOR)
        .map(|s| s.to_string())
        // remove any empty strings as these are not valid tags
        .filter(|s| !s.is_empty())
        .collect()
}
