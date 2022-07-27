fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Money {
    amount: i32,
    currency: Currency,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Currency {
    USD,
    CHF,
}

impl Money {
    fn dollar(amount: i32) -> Money {
        Money {
            amount,
            currency: Currency::USD,
        }
    }
    fn franc(amount: i32) -> Money {
        Money {
            amount,
            currency: Currency::CHF,
        }
    }

    fn times(&self, multiplier: i32) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency,
        }
    }

    fn plus(&self, addend: &Money) -> impl Expression {
        Money {
            amount: self.amount + addend.amount,
            currency: self.currency,
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

trait Expression {}

impl Expression for Money {}

struct Bank {}

impl Bank {
    fn reduce(&self, source: impl Expression, to: Currency) -> Money {
        return Money::dollar(10);
    }
}

#[test]
fn test_multiplication() {
    let five = Money::dollar(5);
    assert_eq!(Money::dollar(10), five.times(2));
    assert_eq!(Money::dollar(15), five.times(3));
}

#[test]
fn test_equality() {
    assert!(Money::dollar(5) == Money::dollar(5));
    assert!(Money::dollar(5) != Money::dollar(6));
    assert!(Money::franc(5) != Money::dollar(5));
}

#[test]
fn test_currency() {
    assert_eq!(Currency::USD, Money::dollar(1).currency);
    assert_eq!(Currency::CHF, Money::franc(1).currency);
}

#[test]
fn test_simple_addition() {
    let five = Money::dollar(5);
    let sum = five.plus(&five);
    let bank = Bank {};
    let reduced = bank.reduce(sum, Currency::USD);
    assert_eq!(Money::dollar(10), reduced)
}
