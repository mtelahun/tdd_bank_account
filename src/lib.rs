use rust_decimal::Decimal;

#[derive(Debug, Default)]
struct BankAccount {
    balance: Decimal,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InsufficientBalance,
}

impl BankAccount {
    pub fn new() -> Self {
        Self {
            balance: Decimal::ZERO,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) -> Decimal {
        self.balance += amount;

        self.balance
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<Decimal, Error> {
        todo!()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::InsufficientBalance => "unsufficient balance",
        };

        write!(f, "operation declined: {msg}")
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use crate::{BankAccount, Error};

    #[test]
    fn given_account_when_amount_deposited_then_new_balance_equals_amount() {
        // Arrange
        let amount = Decimal::new(10000, 2);    // 100.00
        let mut bank_account = BankAccount::new();

        // Act
        let new_balance = bank_account.deposit(amount);

        // Assert
        assert_eq!(new_balance, Decimal::from_str("100.00").unwrap());
    }

    #[test]
    fn given_account_with_balance_when_amount_deposited_then_new_balance_increased_by_amount() {
        // Arrange
        let amount = Decimal::new(10000, 2);    // 100.00
        let mut bank_account = BankAccount::new();
        let _ = bank_account.deposit(amount);

        // Act
        let new_balance = bank_account.deposit(amount);

        // Assert
        assert_eq!(new_balance, Decimal::from_str("200.00").unwrap());
    }

    #[test]
    fn given_account_when_withdrawal_reduces_balance_below_zero_then_insuficient_balance_error() {
        // Arrange
        let amount = Decimal::new(10000, 2);    // 100.00
        let mut bank_account = BankAccount::new();

        // Act
        let result = bank_account.withdraw(amount);

        // Assert
        assert_eq!(
            result.err().unwrap(),
            Error::InsufficientBalance
        );
    }
}