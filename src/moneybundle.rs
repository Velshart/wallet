//use std::fmt::{Display, Formatter};
use core::fmt::Display;
use core::fmt::Formatter;
#[derive(Clone, Copy, Debug)]
pub enum Currency {
    EUR,
    PLN,
    USD,
    RUB
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let currency_str = match self {
            Currency::EUR => "EUR",
            Currency::PLN => "PLN",
            Currency::USD => "USD",
            Currency::RUB => "RUB",
        };
        write!(f, "{}", currency_str)
    }
}

#[derive(Debug)]
pub struct MoneyBundle {
    pub currency: Currency,
    pub amount: u32
}

impl MoneyBundle {

    pub fn new(currency: Currency, amount: u32) -> MoneyBundle {
        MoneyBundle{currency, amount}
    }
    pub fn state(&self) -> String {
        let mut str: String = String::new();
        str.push_str(self.currency.to_string().as_str());
        str.push_str(" ");
        str.push_str(self.amount.to_string().as_str());

        str
    }
}

impl PartialEq for MoneyBundle {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}
