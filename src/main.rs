fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
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

    fn plus(&self, addend: &Money) -> Sum {
        Sum {
            augend: *self,
            addend: *addend,
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

trait Expression {
    fn reduce(&self, to: Currency) -> Money;
}

impl Expression for Money {
    fn reduce(&self, to: Currency) -> Money {
        return Money { ..*self };
    }
}

struct Bank {}

impl Bank {
    fn reduce(&self, source: impl Expression, to: Currency) -> Money {
        return source.reduce(to);
    }
}

struct Sum {
    augend: Money,
    addend: Money,
}

impl Expression for Sum {
    fn reduce(&self, to: Currency) -> Money {
        let amount = self.augend.amount + self.addend.amount;
        return Money {
            amount: amount,
            currency: to,
        };
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

#[test]
fn test_plus_returns_sum() {
    let five = Money::dollar(5);
    let result = five.plus(&five);
    let sum = result as Sum;
    assert_eq!(five, sum.augend);
    assert_eq!(five, sum.addend);
}

#[test]
fn test_reduce_sum() {
    let sum = Sum {
        augend: Money::dollar(3),
        addend: Money::dollar(4),
    };
    let bank = Bank {};
    let result = bank.reduce(sum, Currency::USD);
    assert_eq!(Money::dollar(7), result);
}

#[test]
fn test_reduce_money() {
    let bank = Bank {};
    let result = bank.reduce(Money::dollar(1), Currency::USD);
    assert_eq!(Money::dollar(1), result);
}
