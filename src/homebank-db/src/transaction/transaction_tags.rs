//! Helper functions to handle the processing of [`Transaction`][crate::transaction::transaction::Transaction] tags.

/// The character separating each tag in the HomeBank XML file.
const TAG_SEPARATOR: char = ' ';

/// Get the list of tags for a [`Transaction`][crate::transaction::transaction::Transaction] and parse them.
pub(crate) fn split_tags(s: &str) -> Vec<String> {
    s.split(TAG_SEPARATOR)
        .map(|s| s.to_string())
        // remove any empty strings as these are not valid tags
        .filter(|s| !s.is_empty())
        .collect()
}
