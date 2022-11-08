//! Flags that can be placed on a [`Category`][crate::category::category::Category].

/// Flags that can be placed on a [`Category`][crate::category::category::Category].
#[derive(Debug, PartialEq, Eq)]
pub enum CategoryFlag {
    /// The [`Category`][crate::category::category::Category] is a sub-category of another category.
    /// Equivalent to `GF_SUB`.
    SubCategory,
    
    /// Equivalent to `GF_INCOME`.
    Income,
    
    /// Equivalent to `GF_CUSTOM`.
    Custom,
    
    /// Equivalent to `GF_BUDGET`.
    Budget,
    
    /// Equivalent to `GF_FORCED`.
    Forced,
    
    /// Equivalent to `GF_MIXED`.
    Mixed,
}

/// The set of flags that can be placed on any [`Category`][crate::category::category::Category], stored efficiently.
pub struct CategoryFlags(u8);