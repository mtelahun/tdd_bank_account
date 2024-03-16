use std::{
    io::{self, BufRead, Write},
    str::FromStr,
};

use rust_decimal::Decimal;
use tdd_banking::BankAccount;

fn main() {
    let mut bank_account = BankAccount::new();
    println!("Bank Account created.");

    loop {
        let input = solicit_input(
            "Please enter transaction type(1: Deposit, 2: Withdrawal, 3: Print Statement): ",
        );

        if input == "1" {
            let amount = solicit_input("Enter amount: ");
            let amount = Decimal::from_str(&amount).expect("unable to parse the amount");
            let _ = bank_account.deposit(amount);
        } else if input == "2" {
            let amount = solicit_input("Enter amount: ");
            let amount = Decimal::from_str(&amount).expect("unable to parse the amount");
            match bank_account.withdraw(amount) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{e}");
                    return ();
                }
            }
        } else if input == "3" {
            let statement = bank_account.get_statement();
            println!("|DATE                 | AMOUNT  | BALANCE|");
            println!("|---------------------|---------|--------|");
            for line in statement.lines.iter() {
                println!(
                    " {}  |   {}   | {} ",
                    line.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    line.amount,
                    line.balance
                );
            }
        } else {
            eprintln!("unable to understand input");
            return ();
        }
    }
}

fn solicit_input(prompt: &str) -> String {
    write!(io::stdout(), "{prompt}").expect("failed to write prompt");
    io::stdout().flush().expect("failed to flush prompt");
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("failed to read amount");

    input.trim().to_owned()
}
