//! Categories for each [`Transaction`][crate::transaction::transaction_struct::Transaction].

use super::{CategoryBudget, CategoryError};
use crate::HomeBankDb;
use chrono::NaiveDate;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

/// Categories for each [`Transaction`][crate::transaction::transaction_struct::Transaction].
#[derive(Debug, PartialEq, Clone)]
pub struct Category {
    /// The unique primary key for the category in the database.
    key: usize,

    /// Flags listed on the category.
    ///
    /// The following flags are found in the original codebase:
    /// - `AF_ADDED`
    /// - `AF_CHANGED`
    /// - `AF_CLOSED`
    /// - `AF_NOBUDGET`
    /// - `AF_NOREPORT`
    /// - `AF_NOSUMMARY`
    /// - `AF_OLDBUDGET`
    /// - `ASGF_DOCAT`
    /// - `ASGF_DOMOD`
    /// - `ASGF_DOPAY`
    /// - `ASGF_EXACT`
    /// - `ASGF_OVWCAT`
    /// - `ASGF_OVWMOD`
    /// - `ASGF_OVWPAY`
    /// - `ASGF_REGEX`
    /// - `CF_CUSTOM`
    /// - `FLG_REG_TITLE`
    /// - `FLG_REG_VISUAL`
    /// - `FLG_REG_BALANCE`
    /// - `FLG_REG_SENSITIVE`
    /// - `FLG_REG_`
    /// - `FLT_QSEARCH_MEMO`
    /// - `FLT_QSEARCH_INFO`
    /// - `FLT_QSEARCH_PAYEE`
    /// - `FLT_QSEARCH_CATEGORY`
    /// - `FLT_QSEARCH_TAGS`
    /// - `FLT_QSEARCH_AMOUNT`
    /// - `GF_BUDGET`
    /// - `GF_CUSTOM`
    /// - `GF_FORCED`
    /// - `GF_INCOME`
    /// - `GF_MIXED`
    /// - `GF_SUB`
    /// - `OF_ADDED`
    /// - `OF_AUTO`
    /// - `OF_CHANGED`
    /// - `OF_CHEQ2`
    /// - `OF_INCOME`
    /// - `OF_INTXFER`
    /// - `OF_LIMIT`
    /// - `OF_REMIND`
    /// - `OF_SPLIT`
    /// - `OF_VALID`
    /// - `OLDF_REMIND`
    /// - `OLDF_VALID`
    /// - `TXN_DSPFLG_DUPDST`
    /// - `TXN_DSPFLG_DUPSRC`
    /// - `TXN_DSPFLG_OVER`
    /// - `TXN_DSPFLG_LOWBAL`
    /// - `UF_TITLE`
    /// - `UF_SENSITIVE`
    /// - `UF_VISUAL`
    /// - `UF_REFRESHALL`
    flags: usize,

    /// The name of the category.
    name: String,

    /// A credit or debit budget for this category across all [`Transaction`s][crate::transaction::transaction_struct::Transaction].
    budget: CategoryBudget,

    /// To help when grouping items, categories may be organized into subcategories.
    /// If this is the case, `parent_key` will give the primary key for its parent.
    /// This will be `Some(_)` if this category is a subcategory for some other category.
    ///
    /// # Examples
    ///
    /// A parent `Vehicle` category can be subdivided into `Vehicle:Gasoline` and `Vehicle:Insurance` categories.
    /// The relationship between the categories would look something like:
    ///
    /// ```rust,compile_fail
    /// Category {
    ///     key: 0,
    ///     name: "Vehicle",
    ///     ...
    /// }
    /// Category {
    ///     key: 1,
    ///     name: "Gasoline",
    ///     parent_key: Some(0),
    ///     ...
    /// }
    /// Category {
    ///     key: 2,
    ///     name: "Insurance",
    ///     parent_key: Some(0),
    ///     ...
    /// }
    /// ```
    parent_key: Option<usize>,
}

impl Category {
    /// Create an empty `Category`.
    pub fn empty() -> Self {
        Self {
            key: 0,
            flags: 0,
            name: "".to_string(),
            budget: CategoryBudget::default(),
            parent_key: None,
        }
    }

    /// Create a new `Category`
    pub fn new(key: usize, flags: usize, name: &str, parent_key: Option<usize>) -> Self {
        Self {
            key,
            flags,
            name: name.to_string(),
            budget: CategoryBudget::empty(),
            parent_key,
        }
    }

    /// Retrieve the `Category`'s key
    pub(crate) fn key(&self) -> usize {
        self.key
    }

    /// Retrieve the `Category`'s name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if the `Category` is a child of another category.
    pub fn is_child(&self) -> bool {
        self.parent_key.is_some()
    }

    /// Retrieve the `Category`'s parent category name, if one exists.
    pub fn parent_name<'db>(&self, db: &'db HomeBankDb) -> Option<&'db str> {
        if let Some(idx) = self.parent_key {
            if let Some(parent_cat) = db.categories().get(&idx) {
                Some(parent_cat.name())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Retrieve the `Category`'s name, including the parent category, if one exists.
    pub fn full_name(&self, db: &HomeBankDb) -> String {
        if let Some(idx) = self.parent_key {
            if let Some(parent_cat) = db.categories().get(&idx) {
                format!("{}:{}", parent_cat.name(), self.name())
            } else {
                self.name().to_string()
            }
        } else {
            self.name().to_string()
        }
    }

    /// Retrieve the [`Category`][crate::category::category_struct::Category]'s flags.
    pub fn flags(&self) -> usize {
        self.flags
    }

    /// Set the budget amount for a month or each month.
    pub fn set_budget(&mut self, index: usize, amount: f32) -> Result<(), CategoryError> {
        self.budget.set_budget(index, amount)
    }

    /// Retrieve the `Category`'s budget.
    pub fn budget(&self) -> &CategoryBudget {
        &self.budget
    }

    /// Determine if the `Category` has a budget or not.
    pub fn has_budget(&self) -> bool {
        !self.budget.is_empty()
    }

    /// Retrieve the budget amount for a given month.
    pub fn budget_amount(&self, month: usize) -> Option<f32> {
        self.budget.budget(month)
    }

    /// Retrieve the total budget amount of an interval of time.
    pub fn budget_amount_over_interval(&self, from: NaiveDate, to: NaiveDate) -> Option<f32> {
        self.budget.budget_over_interval(from, to)
    }
}

impl Default for Category {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for Category {
    type Error = CategoryError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut cat = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                // category name
                "name" => {
                    cat.name = i.value.to_string();
                }
                // category key in the database
                "key" => {
                    cat.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CategoryError::InvalidKey),
                    }
                }
                // flags for the category
                "flags" => {
                    cat.flags = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CategoryError::InvalidFlags),
                    }
                }
                // a parent category (if any)
                "parent" => {
                    cat.parent_key = match usize::from_str(&i.value) {
                        Ok(idx) => Some(idx),
                        Err(_) => return Err(CategoryError::InvalidParentKey),
                    }
                }
                // budgeting for each month
                "b0" | "b1" | "b2" | "b3" | "b4" | "b5" | "b6" | "b7" | "b8" | "b9" | "b10"
                | "b11" | "b12" => {
                    let index = match usize::from_str(&i.name.local_name.as_str()[1..]) {
                        Ok(i) => i,
                        Err(_) => return Err(CategoryError::InvalidBudgetProperty),
                    };
                    let amount = match f32::from_str(&i.value) {
                        Ok(v) => v,
                        Err(_) => return Err(CategoryError::InvalidBudgetValue),
                    };
                    cat.set_budget(index, amount)?;
                }
                _ => {}
            }
        }
        Ok(cat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml::{reader::XmlEvent, EventReader};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[track_caller]
    fn check_try_from_single_str(input: &str, expected: Result<Category, CategoryError>) {
        // set up the reader from the input string
        let mut reader = EventReader::from_str(input);

        // skip the XML starting header and parse the first event
        let (_start, first) = (reader.next(), reader.next());

        // get the first event
        if let Ok(XmlEvent::StartElement {
            name, attributes, ..
        }) = first
        {
            if "cat" == name.local_name.as_str() {
                let observed = Category::try_from(attributes);
                assert_eq!(expected, observed);
            } else {
                panic!(
                    "Incorrect category string passed into check. Expected `cat`, found `{:#?}`",
                    name.local_name.as_str()
                );
            }
        } else {
            panic!("Incorrect string passed into check. `{:#?}`", first);
        }
    }

    #[test]
    fn parse_simple_category() {
        let input = r#"<cat key="1" name="Name">"#;
        let expected = Ok(Category {
            key: 1,
            name: "Name".to_string(),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_simple_subcategory() {
        let input = r#"<cat key="2" name="Name" parent="1">"#;
        let expected = Ok(Category {
            key: 2,
            name: "Name".to_string(),
            parent_key: Some(1),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_simple_budget_each_month() {
        let input = r#"<cat key="1" name="Name" b0="-400">"#;
        let expected = Ok(Category {
            key: 1,
            name: "Name".to_string(),
            parent_key: None,
            flags: 0,
            budget: CategoryBudget {
                each_month: Some(-400.0),
                ..Default::default()
            },
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_budget_each_month_with_single_month() {
        let input = r#"<cat key="1" name="Name" b0="-400" b2="-200">"#;
        let expected = Ok(Category {
            key: 1,
            name: "Name".to_string(),
            parent_key: None,
            flags: 0,
            budget: CategoryBudget {
                each_month: Some(-400.0),
                february: Some(-200.0),
                ..Default::default()
            },
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_simple_budget_single_month() {
        let input = r#"<cat key="1" name="Name" b2="-200">"#;
        let expected = Ok(Category {
            key: 1,
            name: "Name".to_string(),
            parent_key: None,
            flags: 0,
            budget: CategoryBudget {
                february: Some(-200.0),
                ..Default::default()
            },
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_non_budget() {
        let cat = Category {
            key: 157,
            parent_key: Some(106),
            flags: 1,
            name: "Parking".to_string(),
            budget: CategoryBudget::empty(),
        };

        assert!(!cat.has_budget());
    }

    #[test]
    fn parse_budget() {
        let cat = Category {
            key: 157,
            parent_key: Some(106),
            flags: 1,
            name: "Parking".to_string(),
            budget: CategoryBudget {
                february: Some(2.0),
                ..Default::default()
            },
        };

        assert!(cat.has_budget());
    }
}
