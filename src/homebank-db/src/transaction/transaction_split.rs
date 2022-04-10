use xml::attribute::OwnedAttribute;

/// The string separator used to denote split transactions
const SPLIT_SEPARATOR: &str = "||";

/// Parse the values stored in a split transaction or template.
pub(crate) fn parse_split_values(att: OwnedAttribute) -> Vec<String> {
    let vals = att
        .value
        .as_str()
        .split(SPLIT_SEPARATOR)
        .map(|s| s.to_string())
        .collect();

    vals
}
