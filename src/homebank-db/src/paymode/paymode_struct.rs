//! Payment method for a [`Transaction`][crate::transaction::transaction::Transaction].

use crate::TransactionError;
use std::str::FromStr;

/// Payment method for a [`Transaction`][crate::transaction::transaction::Transaction].
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PayMode {
    None,
    CreditCard,
    Cheque,
    Cash,
    BankTransfer,
    DebitCard,
    StandingOrder,
    ElectronicPayment,
    Deposit,
    FinancialInstitutionFee,
    DirectDebit,
}

impl Default for PayMode {
    fn default() -> Self {
        Self::None
    }
}

impl TryFrom<usize> for PayMode {
    type Error = TransactionError;

    fn try_from(u: usize) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(PayMode::None),
            1 => Ok(PayMode::CreditCard),
            2 => Ok(PayMode::Cheque),
            3 => Ok(PayMode::Cash),
            4 => Ok(PayMode::BankTransfer),
            5 => Ok(PayMode::DebitCard),
            6 => Ok(PayMode::StandingOrder),
            7 => Ok(PayMode::ElectronicPayment),
            8 => Ok(PayMode::Deposit),
            9 => Ok(PayMode::FinancialInstitutionFee),
            10 => Ok(PayMode::DirectDebit),
            _ => Err(TransactionError::InvalidStatus),
        }
    }
}

impl FromStr for PayMode {
    type Err = TransactionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "none" | "None" => Ok(PayMode::None),
            "1" | "CreditCard" | "Credit" | "credit" => Ok(PayMode::CreditCard),
            "2" | "Cheque" | "Check" | "cheque" | "check" => Ok(PayMode::Cheque),
            "3" | "Cash" | "cash" => Ok(PayMode::Cash),
            "4" | "BankTransfer" | "transfer" => Ok(PayMode::BankTransfer),
            "5" | "DebitCard" | "Debit" | "debit" => Ok(PayMode::DebitCard),
            "6" | "StandingOrder" => Ok(PayMode::StandingOrder),
            "7" | "ElectronicPayment" | "ETransfer" | "eTransfer" | "E-Transfer" | "e-Transfer" | "e-transfer" | "etransfer" => Ok(PayMode::ElectronicPayment),
            "8" | "Deposit" | "deposit" => Ok(PayMode::Deposit),
            "9" | "FIFee" | "Fee" | "fee" => Ok(PayMode::FinancialInstitutionFee),
            "10" | "DirectDebit" => Ok(PayMode::DirectDebit),
            _ => Err(TransactionError::InvalidPayMode),
        }
    }
}
