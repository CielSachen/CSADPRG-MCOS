/*
 * Last Names: Panaligan
 * Language: Rust
 * Paradigm(s): Procedural, Object-Oriented, Functional
 */

use std::{
    collections::HashMap,
    fmt,
    io::{self, Write},
};

fn print_choices<T: fmt::Display>(choices: &[T]) {
    for (i, val) in choices.iter().enumerate() {
        println!("[{}] {val}", i + 1)
    }
}

fn prompt(msg: &str) -> String {
    print!("{msg}");

    io::stdout().flush().expect("Failed to flush the output string...");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input string...");

    input.trim().to_string()
}

const CURRENCIES: [&str; 6] = [
    "Philippine Peso (PHP)",
    "United States Dollar (USD)",
    "Japanese Yen (JPY)",
    "British Pound Sterling (GBP)",
    "Euro (EUR)",
    "Chinese Yuan Renminni (CNY)",
];

const TRANSACTION_TITLES: [&str; 6] = [
    "Register Account Name",
    "Deposit Amount",
    "Withdraw Amount",
    "Currency Exchange",
    "Record Exchange Rates",
    "Show Interest Amount",
];

#[derive(PartialEq)]
struct Account {
    name: String,
    balance: f64,
    currency: String,
}
impl Account {
    fn new(name: String) -> Account {
        Account {
            name,
            balance: 0.0,
            currency: String::from("PHP"),
        }
    }
}

fn deposit_balance(account: &mut Account) {
    println!("Current Balance: {}", account.balance);
    println!("Currency: {}", account.currency);

    println!();

    if let Ok(amount) = prompt("Deposit Amount: ").parse::<f64>() {
        account.balance += amount;

        println!("Updated Balance: {}", account.balance);
    } else {
        println!("Deposit amount must be a floating point number!");
    }
}

fn withdraw_balance(account: &mut Account) {
    println!("Current Balance: {}", account.balance);
    println!("Currency: {}", account.currency);

    println!();

    if let Ok(amount) = prompt("Withdraw Amount: ").parse::<f64>() {
        account.balance -= amount;

        println!("Updated Balance: {}", account.balance);
    } else {
        println!("Withdraw amount must be a floating point number!");
    }
}

fn exchange_currencies(rates: &HashMap<&str, f64>) {
    println!("Source Currency Options:");
    print_choices(&CURRENCIES);

    println!();

    let src_idx = match prompt("Source Currency: ").parse::<usize>() {
        Ok(idx) => idx - 1,
        Err(_) => {
            println!("ID must be a positive whole number (integer)!");

            return;
        }
    };

    if src_idx >= CURRENCIES.len() {
        println!("No currency with this ID exists!");

        return;
    }

    let src_amount = match prompt("Source Amount: ").parse::<f64>() {
        Ok(amount) => amount,
        Err(_) => {
            println!("Amount must be a floating point number!");

            return;
        }
    };

    println!();

    println!("Exchanged Currency Options:");
    print_choices(&CURRENCIES);

    println!();

    let exchange_idx = match prompt("Exchange Currency: ").parse::<usize>() {
        Ok(idx) => idx - 1,
        Err(_) => {
            println!("ID must be a positive whole number (integer)!");

            return;
        }
    };

    if exchange_idx >= CURRENCIES.len() {
        println!("No currency with this ID exists!");

        return;
    }

    let src_php_amount = src_amount * rates[CURRENCIES[src_idx - 1]];
    let exchange_amount = if exchange_idx == 1 {
        src_php_amount
    } else {
        src_php_amount * rates[CURRENCIES[exchange_idx - 1]]
    };

    println!("Exchange Amount: {exchange_amount}");
}

fn set_exchange_rate(rates: &mut HashMap<&str, f64>) {
    print_choices(&CURRENCIES);

    println!();

    let idx = match prompt("Select Foreign Currency: ").parse::<usize>() {
        Ok(idx) => idx - 1,
        Err(_) => {
            println!("ID must be a positive whole number (integer)!");

            return;
        }
    };

    if idx >= CURRENCIES.len() {
        println!("No currency with this ID exists!");

        return;
    }

    let rate = match prompt("Exchange Rate: ").parse::<f64>() {
        Ok(rate) => rate,
        Err(_) => {
            println!("Amount must be a floating point number!");

            return;
        }
    };

    rates.insert(CURRENCIES[idx - 1], rate);
}

const ANNUAL_INTEREST_RATE: f64 = 0.05;

fn calculate_interest(account: &Account) {
    let mut curr_balance = account.balance;

    println!("Current Balance: {curr_balance}");
    println!("Currency: {}", account.currency);
    println!("Interest Rate: {}%", (ANNUAL_INTEREST_RATE * 100.0) as i32);

    println!();

    if let Ok(day_cnt) = prompt("Total Number of Days: ").parse::<i32>() {
        println!();

        println!("Day | Interest | Balance |");

        let daily_interest = ANNUAL_INTEREST_RATE / 365.0;

        for i in 1..=day_cnt {
            curr_balance += curr_balance * daily_interest;

            println!(
                "{day:<3.2} | {interest:<8.2} | {balance:<7.2} |",
                day = i,
                interest = daily_interest,
                balance = curr_balance
            );
        }
    } else {
        println!("ID must be a whole number (integer)!");
    }
}

fn main() {
    let mut accounts = Vec::new();
    let mut exchange_rates = HashMap::<&str, f64>::new();

    for currency in CURRENCIES.iter().skip(1) {
        exchange_rates.insert(currency, 1.0);
    }

    'main_menu: loop {
        println!("Select Transaction:");
        print_choices(&TRANSACTION_TITLES);

        println!();

        let chosen_idx = prompt("> ").parse::<usize>().unwrap_or_default();

        println!();

        if chosen_idx > 0 && chosen_idx <= TRANSACTION_TITLES.len() {
            println!("{}", TRANSACTION_TITLES[chosen_idx - 1]);
        }

        match chosen_idx {
            1 => {
                let account = Account::new(prompt("Account Name: "));

                if !accounts.contains(&account) {
                    accounts.push(account);
                } else {
                    println!("An account with this name already exists!");
                }
            }
            2 | 3 => {
                if let Some(account) = accounts.iter_mut().find(|a| a.name == prompt("Account Name: ")) {
                    if chosen_idx == 2 {
                        deposit_balance(account);
                    } else {
                        withdraw_balance(account);
                    }
                } else {
                    println!("No account with this name exists!");
                }
            }
            4 => 'currency_exchange: loop {
                exchange_currencies(&exchange_rates);

                println!();

                'repeat_prompt: loop {
                    let is_repeating = prompt("Convert another currency? (Y/N): ").to_uppercase();

                    if is_repeating == "Y" {
                        println!();

                        break 'repeat_prompt;
                    } else if is_repeating == "N" {
                        break 'currency_exchange;
                    } else {
                        println!("Only accepting a [Y]es or [N]o answer!");

                        println!();
                    }
                }
            },
            5 => {
                println!();

                set_exchange_rate(&mut exchange_rates);
            }
            6 => {
                if let Some(account) = accounts.iter().find(|a| a.name == prompt("Account Name: ")) {
                    calculate_interest(account);
                } else {
                    println!("No account with this name exists!");
                }
            }
            _ => {
                println!("No transaction with this ID exists!")
            }
        }

        println!();

        'exit_prompt: loop {
            let is_continuing = prompt("Back to the Main Menu (Y/N): ").to_uppercase();

            if is_continuing == "Y" {
                println!();

                break 'exit_prompt;
            } else if is_continuing == "N" {
                break 'main_menu;
            } else {
                println!("Only accepting a [Y]es or [N]o answer!");

                println!();
            }
        }
    }
}
