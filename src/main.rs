use crate::moneybundle::Currency;

mod moneybundle;
mod wallet {
    use std::cmp::PartialEq;
    use crate::moneybundle;
    use crate::moneybundle::Currency;

    pub struct Wallet {
        money: Vec<moneybundle::MoneyBundle>
    }

    impl PartialEq for Currency {
        fn eq(&self, other: &Self) -> bool {
            self.to_string() == other.to_string()
        }
    }

    impl Wallet {
        pub fn new() -> Self {
            Wallet{money: Vec::new()}
        }

        pub fn add(&mut self, currency: Currency, amount: u32) {

            let ind: Option<usize> = self.find_currency(currency);

            if ind.is_some() {
                self.money[ind.unwrap()].amount+=amount;
            }else {
                self.money.push(moneybundle::MoneyBundle::new(currency, amount));
            }
        }

        pub fn remove(&mut self, currency: Currency, amount: u32) {
            let ind: Option<usize> = self.find_currency(currency);

            if ind.is_some() {
                if amount > self.money[ind.unwrap()].amount {
                    self.money[ind.unwrap()].amount = 0;
                }else {

                self.money[ind.unwrap()].amount-=amount;
                }
            }
        }

        pub fn state(&self) {
            for i in &self.money {
                println!("{}", i.state());
            }
        }

        fn find_currency(&self, currency: Currency) -> Option<usize> {
            let mut result: usize = 0;

            let mut found: bool = false;
            for i in &self.money {
                if i.currency == currency {
                    found = true;
                    break;
                }
                result+=1;
            }
            if found {
                return Some(result);
            }
            None
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::moneybundle::{Currency, MoneyBundle};
        use crate::wallet;

        #[test]
        fn test_add1() {
            let mut w = wallet::Wallet::new();
            w.add(Currency::PLN, 5);

            assert_eq!(w.money[0], MoneyBundle::new(Currency::PLN, 5));
        }
        #[test]
        fn test_add2() {
            let mut w = wallet::Wallet::new();
            w.add(Currency::PLN, 5);

            w.add(Currency::PLN, 1);

            assert_eq!(w.money[0], MoneyBundle::new(Currency::PLN, 6));
        }

        #[test]
        fn test_remove1() {
            let mut w = wallet::Wallet::new();
            w.add(Currency::PLN, 5);

            w.remove(Currency::PLN, 1);

            assert_eq!(w.money[0], MoneyBundle::new(Currency::PLN, 4));
        }

        #[test]
        fn test_remove2() {
            let mut w = wallet::Wallet::new();
            w.add(Currency::PLN, 5);

            w.remove(Currency::PLN, 6);

            assert_eq!(w.money[0], MoneyBundle::new(Currency::PLN, 0));
        }
    }
}

fn main() {
    let mut wallet: wallet::Wallet = wallet::Wallet::new();

    wallet.add(Currency::EUR, 20);
    wallet.add(Currency::EUR, 1);

    wallet.add(Currency::RUB, 5);

    wallet.state();
}
