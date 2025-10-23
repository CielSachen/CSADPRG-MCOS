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

const CURRENCY_CNT: usize = 6;
const CURRENCIES_TITLES: [&str; CURRENCY_CNT] = [
    "Philippine Peso (PHP)",
    "United States Dollar (USD)",
    "Japanese Yen (JPY)",
    "British Pound Sterling (GBP)",
    "Euro (EUR)",
    "Chinese Yuan Renminni (CNY)",
];
const CURRENCIES_CODES: [&str; CURRENCY_CNT] = ["PHP", "USD", "JPY", "GBP", "EUR", "CNY"];

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

fn convert_currency(amount: f64, src: &&str, dest: &&str, rates: &HashMap<&str, f64>) -> f64 {
    let src_php_amount = if *src == "PHP" { amount } else { amount * rates[src] };

    if *dest == "PHP" {
        src_php_amount
    } else {
        src_php_amount * rates[dest]
    }
}

fn deposit_balance(account: &mut Account, rates: &HashMap<&str, f64>) {
    println!("Current Balance: {}", account.balance);

    let currency = prompt("Currency: ").to_uppercase();

    if !CURRENCIES_CODES.iter().any(|c| *c == currency) {
        println!("No currency with this code exists!");

        return;
    }

    println!();

    if let Ok(amount) = prompt("Deposit Amount: ").parse::<f64>() {
        account.balance += if currency == "PHP" {
            amount
        } else {
            convert_currency(amount, &currency.as_str(), &"PHP", rates)
        };

        println!("Updated Balance: {}", account.balance);
    } else {
        println!("Deposit amount must be a floating point number!");
    }
}

fn withdraw_balance(account: &mut Account, rates: &HashMap<&str, f64>) {
    println!("Current Balance: {}", account.balance);

    let currency = prompt("Currency: ").to_uppercase();

    if !CURRENCIES_CODES.iter().any(|c| *c == currency) {
        println!("No currency with this code exists!");

        return;
    }

    println!();

    if let Ok(mut amount) = prompt("Withdraw Amount: ").parse::<f64>() {
        amount = if currency == "PHP" {
            amount
        } else {
            convert_currency(amount, &currency.as_str(), &"PHP", rates)
        };

        if account.balance - amount < 0.0 {
            println!("Withdraw amount must be less than the current balance!");

            return;
        }

        account.balance -= amount;

        println!("Updated Balance: {}", account.balance);
    } else {
        println!("Withdraw amount must be a floating point number!");
    }
}

fn exchange_currencies(rates: &HashMap<&str, f64>) {
    println!("Source Currency Options:");
    print_choices(&CURRENCIES_TITLES);

    println!();

    let src_idx = match prompt("Source Currency: ").parse::<usize>() {
        Ok(idx) => idx - 1,
        Err(_) => {
            println!("ID must be a positive whole number (integer)!");

            return;
        }
    };

    if src_idx >= CURRENCY_CNT {
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
    print_choices(&CURRENCIES_TITLES);

    println!();

    let exchange_idx = match prompt("Exchange Currency: ").parse::<usize>() {
        Ok(idx) => idx - 1,
        Err(_) => {
            println!("ID must be a positive whole number (integer)!");

            return;
        }
    };

    if exchange_idx >= CURRENCY_CNT {
        println!("No currency with this ID exists!");

        return;
    }

    println!(
        "Exchange Amount: {}",
        convert_currency(
            src_amount,
            &CURRENCIES_CODES[src_idx],
            &CURRENCIES_CODES[exchange_idx],
            rates
        )
    );
}

fn set_exchange_rate(rates: &mut HashMap<&str, f64>) {
    print_choices(&CURRENCIES_TITLES[1..]);

    println!();

    let idx = match prompt("Select Foreign Currency: ").parse::<usize>() {
        Ok(idx) => idx,
        Err(_) => {
            println!("ID must be a positive whole number (integer)!");

            return;
        }
    };

    if idx >= CURRENCY_CNT {
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

    rates.insert(CURRENCIES_CODES[idx], rate);
}

const ANNUAL_INTEREST_RATE: f64 = 0.05;

fn calculate_interest(account: &Account) {
    let mut balance = account.balance;

    println!("Current Balance: {balance}");
    println!("Currency: {}", account.currency);
    println!("Interest Rate: {}%", (ANNUAL_INTEREST_RATE * 100.0) as i32);

    println!();

    if let Ok(day_cnt) = prompt("Total Number of Days: ").parse::<u32>() {
        println!();

        println!("Day | Interest | Balance |");

        let daily_interest = ANNUAL_INTEREST_RATE / 365.0;

        for i in 1..=day_cnt {
            balance += balance * daily_interest;

            println!(
                "{day:<3} | {interest:<8.2} | {balance:<7.2} |",
                day = i,
                interest = daily_interest,
                balance = balance
            );
        }
    } else {
        println!("Number must be a positive whole number (integer)!");
    }
}

fn main() {
    let mut accounts = Vec::new();
    let mut exchange_rates = HashMap::<&str, f64>::new();

    for code in CURRENCIES_CODES.iter().skip(1) {
        exchange_rates.insert(code, 1.0);
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
                        deposit_balance(account, &exchange_rates);
                    } else {
                        withdraw_balance(account, &exchange_rates);
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
