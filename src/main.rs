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
        write!(
            io::stdout(),
            "Please enter transaction type(1: Deposit, 2: Withdrawal, 3: Print Statement): "
        )
        .expect("failed to write prompt");
        io::stdout().flush().expect("failed to flush output buffer");

        let mut input = String::new();
        io::stdin()
            .lock()
            .read_line(&mut input)
            .expect("failed to read input");
        let input = input.trim();

        if input == "1" {
            write!(io::stdout(), "Enter amount: ").expect("failed to write amount prompt");
            io::stdout().flush().expect("failed to flush amount prompt");
            let mut amount = String::new();
            io::stdin()
                .lock()
                .read_line(&mut amount)
                .expect("failed to read amount");
            let amount = Decimal::from_str(&amount.trim()).expect("unable to parse the amount");
            let _ = bank_account.deposit(amount);
        } else if input == "2" {
            write!(io::stdout(), "Enter amount: ").expect("failed to write amount prompt");
            io::stdout().flush().expect("failed to flush amount prompt");
            let mut amount = String::new();
            io::stdin()
                .lock()
                .read_line(&mut amount)
                .expect("failed to read amount");
            let amount = Decimal::from_str(&amount.trim()).expect("unable to parse the amount");
            match bank_account.withdraw(amount) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{e}");
                    return ()
                },
            }
        } else if input == "3" {
            let statement = bank_account.get_statement();
            println!("|DATE                 | AMOUNT  | BALANCE|");
            println!("|---------------------|---------|--------|");
            for line in statement.lines.iter() {
                println!(" {}  |   {}   | {} ", line.timestamp.format("%Y-%m-%d %H:%M:%S"), line.amount, line.balance);
            }
        } else {
            eprintln!("unable to understand input");
            return ();
        }
    }
}
