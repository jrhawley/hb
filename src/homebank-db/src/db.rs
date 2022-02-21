//! Data structure for the HomeBank database.

use crate::{currency::Currency, Account, Category, Favourite, Group, Payee, Transaction};
use semver::Version;

#[derive(Debug)]
pub struct HomeBankDBProperties {
    title: String,
    currency_idx: usize,
    car_category_idx: usize,
    auto_smode: usize,
    auto_weekday: usize,
}

#[derive(Debug)]
pub struct HomeBankDB {
    xml_schema: Version,
    homebank_version: Version,
    properties: HomeBankDBProperties,
    currencies: Vec<Currency>,
    groups: Vec<Group>,
    accounts: Vec<Account>,
    payees: Vec<Payee>,
    categories: Vec<Category>,
    favourites: Vec<Favourite>,
    transactions: Vec<Transaction>,
}
