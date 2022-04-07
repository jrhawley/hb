//! Handle payment methods and pay modes for transactions.

use crate::TransactionError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
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
            "1" | "CreditCard" => Ok(PayMode::CreditCard),
            "2" | "Cheque" | "Check" => Ok(PayMode::Cheque),
            "3" | "Cash" => Ok(PayMode::Cash),
            "4" | "BankTransfer" => Ok(PayMode::BankTransfer),
            "5" | "DebitCard" => Ok(PayMode::DebitCard),
            "6" | "StandingOrder" => Ok(PayMode::StandingOrder),
            "7" | "ElectronicPayment" => Ok(PayMode::ElectronicPayment),
            "8" | "Deposit" => Ok(PayMode::Deposit),
            "9" | "FIFee" => Ok(PayMode::FinancialInstitutionFee),
            "10" | "DirectDebit" => Ok(PayMode::DirectDebit),
            _ => Err(TransactionError::InvalidPayMode),
        }
    }
}
