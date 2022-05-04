//! A budget for a given category.

use crate::CategoryError;

#[derive(Debug, PartialEq, Clone)]
pub struct CategoryBudget {
    pub each_month: Option<f32>,
    pub january: Option<f32>,
    pub february: Option<f32>,
    pub march: Option<f32>,
    pub april: Option<f32>,
    pub may: Option<f32>,
    pub june: Option<f32>,
    pub july: Option<f32>,
    pub august: Option<f32>,
    pub september: Option<f32>,
    pub october: Option<f32>,
    pub november: Option<f32>,
    pub december: Option<f32>,
}

impl CategoryBudget {
    /// Create a new budget
    pub fn new() -> Self {
        Self::empty()
    }

    /// Create an empty budget
    pub fn empty() -> Self {
        Self {
            each_month: None,
            january: None,
            february: None,
            march: None,
            april: None,
            may: None,
            june: None,
            july: None,
            august: None,
            september: None,
            october: None,
            november: None,
            december: None,
        }
    }

    /// Check if there is a budget in the first place
    pub fn is_empty(&self) -> bool {
        let non_budget = Self::empty();
        *self == non_budget
    }

    /// Set the budget amount for a month or each month
    pub fn set_budget(&mut self, index: usize, amount: f32) -> Result<(), CategoryError> {
        match index {
            0 => self.each_month = Some(amount),
            1 => self.january = Some(amount),
            2 => self.february = Some(amount),
            3 => self.march = Some(amount),
            4 => self.april = Some(amount),
            5 => self.may = Some(amount),
            6 => self.june = Some(amount),
            7 => self.july = Some(amount),
            8 => self.august = Some(amount),
            9 => self.september = Some(amount),
            10 => self.october = Some(amount),
            11 => self.november = Some(amount),
            12 => self.december = Some(amount),
            _ => return Err(CategoryError::InvalidBudgetProperty),
        }

        Ok(())
    }

    /// Get the budget amount for the given month.
    ///
    /// Returns `None` for a month index that is not within 1 - 12 (inclusive).
    pub fn budget(&self, month: usize) -> Option<f32> {
        if month == 0 || month > 12 {
            return None;
        }

        // if there is a global budget per month, return that
        if let Some(val) = self.each_month {
            return Some(val);
        }

        match month {
            1 => self.january,
            2 => self.february,
            3 => self.march,
            4 => self.april,
            5 => self.may,
            6 => self.june,
            7 => self.july,
            8 => self.august,
            9 => self.september,
            10 => self.october,
            11 => self.november,
            12 => self.december,
            _ => return None,
        }
    }
}

impl Default for CategoryBudget {
    fn default() -> Self {
        Self::empty()
    }
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
    fn check_budget_amount(input: (&CategoryBudget, usize), expected: Option<f32>) {
        let budget = input.0;
        let month = input.1;
        let observed = budget.budget(month);

        assert_eq!(expected, observed);
    }

    #[test]
    fn each_month_overrides_none() {
        let budget = CategoryBudget {
            each_month: Some(100.0),
            ..Default::default()
        };
        let expected = Some(100.0);

        // check that it's equal for all allowable months
        for i in 1..=12 {
            check_budget_amount((&budget, i), expected);
        }
    }

    #[test]
    fn outside_1_12_is_none() {
        let budget = CategoryBudget {
            each_month: Some(100.0),
            ..Default::default()
        };

        check_budget_amount((&budget, 0), None);
        check_budget_amount((&budget, 13), None);
    }

    #[test]
    fn each_month_overrides_any_month() {
        let budget = CategoryBudget {
            each_month: Some(100.0),
            january: Some(1.0),
            february: Some(2.0),
            march: Some(3.0),
            april: Some(4.0),
            may: Some(5.0),
            june: Some(6.0),
            july: Some(7.0),
            august: Some(8.0),
            september: Some(9.0),
            october: Some(10.0),
            november: Some(11.0),
            december: Some(12.0),
        };
        let expected = Some(100.0);

        // check that it's equal for all allowable months
        for i in 1..=12 {
            check_budget_amount((&budget, i), expected);
        }
    }

    #[test]
    fn single_month() {
        let budget = CategoryBudget {
            each_month: None,
            january: Some(1.0),
            february: Some(2.0),
            march: Some(3.0),
            april: Some(4.0),
            may: Some(5.0),
            june: Some(6.0),
            july: Some(7.0),
            august: Some(8.0),
            september: Some(9.0),
            october: Some(10.0),
            november: Some(11.0),
            december: Some(12.0),
        };

        // check that it's equal for all allowable months
        for i in 1..=12 {
            check_budget_amount((&budget, i), Some(i as f32));
        }
    }

    #[test]
    fn check_no_budget() {
        let non_budget = CategoryBudget::empty();
        let observed = non_budget.is_empty();

        assert_eq!(true, observed);
    }

    #[test]
    fn check_some_budget_each_month() {
        let budget = CategoryBudget {
            each_month: Some(1.0),
            ..Default::default()
        };
        let observed = budget.is_empty();

        assert_eq!(false, observed);
    }

    #[test]
    fn check_some_budget_one_month() {
        let budget = CategoryBudget {
            january: Some(1.0),
            ..Default::default()
        };
        let observed = budget.is_empty();

        assert_eq!(false, observed);
    }
}
