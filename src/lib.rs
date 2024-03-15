use rust_decimal::Decimal;

#[derive(Debug, Default)]
struct BankAccount {
    balance: Decimal,
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
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use crate::BankAccount;

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
}