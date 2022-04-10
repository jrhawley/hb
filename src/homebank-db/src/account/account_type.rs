//! Account types

use super::AccountError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum AccountType {
    None,
    Bank,
    Cash,
    Asset,
    CreditCard,
    Liability,
    Chequing,
    Savings,
}

impl TryFrom<usize> for AccountType {
    type Error = AccountError;

    fn try_from(u: usize) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(AccountType::None),
            1 => Ok(AccountType::Bank),
            2 => Ok(AccountType::Cash),
            3 => Ok(AccountType::Asset),
            4 => Ok(AccountType::CreditCard),
            5 => Ok(AccountType::Liability),
            6 => Ok(AccountType::Chequing),
            7 => Ok(AccountType::Savings),
            _ => Err(AccountError::InvalidType),
        }
    }
}

impl FromStr for AccountType {
    type Err = AccountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" | "none" | "N" | "n" => Ok(AccountType::None),
            "Bank" | "bank" | "B" | "b" => Ok(AccountType::Bank),
            "Cash" | "cash" | "Ca" | "ca" => Ok(AccountType::Cash),
            "Asset" | "asset" | "A" | "a" => Ok(AccountType::Asset),
            "Credit" | "CreditCard" | "credit" | "creditcard" | "cc" => Ok(AccountType::CreditCard),
            "Liability" | "liability" | "L" | "l" => Ok(AccountType::Liability),
            "Chequing" | "Checking" | "chequing" | "checking" | "Ch" | "ch" => {
                Ok(AccountType::Chequing)
            }
            "Savings" | "savings" | "S" | "s" => Ok(AccountType::Savings),
            _ => Err(AccountError::InvalidType),
        }
    }
}
