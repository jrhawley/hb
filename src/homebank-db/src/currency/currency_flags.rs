//! Flags that can be placed on a [`Currency`][crate::currency::currency::Currency].

/// Flags that can be placed on a [`Currency`][crate::currency::currency::Currency].
#[derive(Debug, PartialEq, Eq)]
pub enum CurrencyFlag {
    /// The [`Currency`][crate::currency::currency::Currency] is a custom one provided by the user.
    /// Equivalent to `CF_CUSTOM`.
    Custom,
}

/// The set of flags that can be placed on a [`Currency`][crate::currency::currency::Currency], stored efficiently.
pub struct CurrencyFlags(u8);
