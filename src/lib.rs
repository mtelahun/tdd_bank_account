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

    pub fn deposit(&self, amount: Decimal) -> Decimal {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use crate::BankAccount;

    #[test]
    fn given_account_when_deposit_amount_then_increase_balance_by_amount() {
        // Arrange
        let amount = Decimal::new(10000, 2);    // 100.00
        let bank_account = BankAccount::new();

        // Act
        let new_balance = bank_account.deposit(amount);

        // Assert
        assert_eq!(new_balance, Decimal::from_str("100.00").unwrap());
    }
}