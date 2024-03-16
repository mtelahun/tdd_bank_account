use chrono::{DateTime, Local, NaiveDateTime};
use rust_decimal::Decimal;

#[derive(Debug, Default)]
struct BankAccount {
    balance: Decimal,
}

#[derive(Debug, Default)]
pub struct BankStatement {
    date_created: DateTime<Local>,
    lines: Vec<StatementLine>,
}

#[derive(Debug)]
pub struct StatementLine {
    timestamp: DateTime<Local>,
    transaction_type: TransactionType,
    amount: Decimal,
}

#[derive(Debug)]
pub enum TransactionType {
    Debit,
    Credit,
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
        if self.balance - amount < Decimal::ZERO {
            return Err(Error::InsufficientBalance);
        }

        self.balance -= amount;

        Ok(self.balance)
    }

    pub fn get_statement(&self) -> BankStatement {
        BankStatement {
            date_created: Local::now(),
            lines: Vec::<StatementLine>::new(),
        }
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
    use std::{arch::x86_64::_mm_aesimc_si128, str::FromStr};

    use rust_decimal::Decimal;

    use crate::{BankAccount, Error};

    #[test]
    fn given_account_when_amount_deposited_then_new_balance_equals_amount() {
        // Arrange
        let amount = Decimal::new(10000, 2); // 100.00
        let mut bank_account = BankAccount::new();

        // Act
        let new_balance = bank_account.deposit(amount);

        // Assert
        assert_eq!(new_balance, Decimal::from_str("100.00").unwrap());
    }

    #[test]
    fn given_account_with_balance_when_amount_deposited_then_new_balance_increased_by_amount() {
        // Arrange
        let amount = Decimal::new(10000, 2); // 100.00
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
        let amount = Decimal::new(10000, 2); // 100.00
        let mut bank_account = BankAccount::new();

        // Act
        let result = bank_account.withdraw(amount);

        // Assert
        assert_eq!(result.err().unwrap(), Error::InsufficientBalance);
    }

    #[test]
    fn given_account_when_withdraw_amount_then_balance_decreases_by_amount() {
        // Arrange
        let amount = Decimal::new(10000, 2); // 100.00
        let mut bank_account = BankAccount::new();
        let _ = bank_account.deposit(amount);

        // Act
        let new_balance = bank_account.withdraw(amount).expect("failed to withdraw");

        // Assert
        assert_eq!(new_balance, Decimal::ZERO);
    }

    #[test]
    fn given_account_with_zero_transactions_when_get_statement_then_return_empty_list() {
        // Arrange
        let bank_account = BankAccount::new();

        // Act
        let statement = bank_account.get_statement();

        // Assert
        assert_eq!(
            statement.lines.len(),
            0,
            "given a new bank account, the bank statement is empty"
        );
    }
}
