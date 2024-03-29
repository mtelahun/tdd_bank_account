use chrono::{DateTime, Local};
use rust_decimal::Decimal;

#[derive(Debug, Default)]
pub struct BankAccount {
    transactions: Vec<Transaction>,
}

#[derive(Debug, Default)]
pub struct BankStatement {
    pub date_created: DateTime<Local>,
    pub lines: Vec<StatementLine>,
}

#[derive(Debug)]
pub struct Transaction {
    timestamp: DateTime<Local>,
    transaction_type: TransactionType,
    amount: Decimal,
}

#[derive(Debug)]
pub struct StatementLine {
    pub timestamp: DateTime<Local>,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub balance: Decimal,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
            transactions: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: Decimal) -> Decimal {
        self.transactions.push(Transaction {
            timestamp: Local::now(),
            transaction_type: TransactionType::Credit,
            amount,
        });

        self.get_balance()
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<Decimal, Error> {
        let balance = self.get_balance();
        if balance - amount < Decimal::ZERO {
            return Err(Error::InsufficientBalance);
        }

        self.transactions.push(Transaction {
            timestamp: Local::now(),
            transaction_type: TransactionType::Debit,
            amount,
        });

        Ok(self.get_balance())
    }

    pub fn get_statement(&self) -> BankStatement {
        let mut statement = BankStatement {
            date_created: Local::now(),
            lines: Vec::<StatementLine>::new(),
        };
        let mut balance = Decimal::ZERO;
        for tx in self.transactions.iter() {
            match tx.transaction_type {
                TransactionType::Debit => balance -= tx.amount,
                TransactionType::Credit => balance += tx.amount,
            }
            statement.lines.push(StatementLine {
                timestamp: tx.timestamp,
                transaction_type: tx.transaction_type,
                amount: tx.amount,
                balance,
            });
        }

        statement
    }

    fn get_balance(&self) -> Decimal {
        let credits: Decimal = self
            .transactions
            .iter()
            .filter(|tx| tx.transaction_type == TransactionType::Credit)
            .map(|tx| tx.amount)
            .sum();

        let debits: Decimal = self
            .transactions
            .iter()
            .filter(|tx| tx.transaction_type == TransactionType::Debit)
            .map(|tx| tx.amount)
            .sum();

        credits - debits
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

    use crate::{BankAccount, Error, TransactionType};

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
    fn given_account_with_zero_transactions_when_get_statement_then_return_empty_statement() {
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

    #[test]
    fn given_account_with_transactions_when_get_statement_then_return_statement_with_transactions()
    {
        // Arrange
        let mut bank_account = BankAccount::new();
        let _ = bank_account.deposit(Decimal::new(20000, 2));
        let _ = bank_account.withdraw(Decimal::new(20000, 2));

        // Act
        let statement = bank_account.get_statement();

        // Assert
        assert_eq!(
            statement.lines.len(),
            2,
            "the bank statement contains 2 transactions"
        );
        assert_eq!(
            statement.lines[0].amount,
            Decimal::new(20000, 2),
            "bank statement line 1 amount is 200.00"
        );
        assert_eq!(
            statement.lines[0].transaction_type,
            TransactionType::Credit,
            "bank statement line 1 transaction is 'Credit'"
        );
        assert_eq!(
            statement.lines[0].balance,
            Decimal::new(20000, 2),
            "bank statement line 1 balance is 200.00"
        );
        assert_eq!(
            statement.lines[1].amount,
            Decimal::new(20000, 2),
            "bank statement line 2 amount is 200.00"
        );
        assert_eq!(
            statement.lines[1].transaction_type,
            TransactionType::Debit,
            "bank statement line 2 transaction is 'Debit'"
        );
        assert_eq!(
            statement.lines[1].balance,
            Decimal::ZERO,
            "bank statement line 2 balance is 0.00"
        );
    }
}
