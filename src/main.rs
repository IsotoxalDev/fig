use clap::Parser;
use colored::*;
use currency_rs::Currency;
use rustyline::DefaultEditor;

mod cli;
mod fs;

use cli::{FigArgs, FigCommands};
use fs::FigData;

const APP_NAME: &'static str = "fig";

fn print_amount(text: &str, bal: Currency) {
    println!(
        "{}: {}",
        text.bright_blue().bold(),
        bal.format().bright_yellow().bold()
    )
}

fn get_amt(of: &str, amt: Option<f64>, r: &mut DefaultEditor) -> f64 {
    if let Some(a) = amt {
        a
    } else {
        loop {
            if let Ok(inp) = r.readline(&format!(
                "{}: ",
                format!("Enter amount to {}", of).bright_green().bold()
            )) {
                if let Ok(val) = inp.parse::<f64>() {
                    break val;
                }
            }
        }
    }
}

fn set_balance(mut bal: Currency, amt: Currency, mut data: FigData, text: &str, sub: bool) {
    if amt.value() == 0.0 {
        print_amount(text, amt);
        print_amount("Balance", bal);
        return;
    }
    bal = bal.add(amt.value() * if sub { -1.0 } else { 1.0 });
    data.balance(bal.value());
    data.add_transaction(sub, amt.value());
    fs::store_data(data);
    print_amount(text, amt);
    print_amount("Balance", bal);
}

fn main() {
    let mut r = DefaultEditor::new().unwrap();
    let (data, config) = fs::set_fs();
    let args = FigArgs::parse();
    let opt = config.get_opts();
    let bal = Currency::new_float(data.get_balance(), Some(opt.clone()));
    if let Some(command) = args.command {
        match command {
            FigCommands::Add { amount } => {
                let amt = Currency::new_float(get_amt("add", amount, &mut r), Some(opt.clone()));
                set_balance(bal, amt, data, "Adding", false)
            }
            FigCommands::Take { amount } => {
                let amt = Currency::new_float(get_amt("take", amount, &mut r), Some(opt.clone()));
                set_balance(bal, amt, data, "Taking", true)
            }
            FigCommands::Log => {
                //pager::Pager::new().setup();
                let mut bal = Currency::new_float(0.0, Some(opt.clone()));
                if data.get_transactions().is_empty() {
                    println! {"{}", "No recorded transactions".bright_purple().bold()}
                    return;
                }
                for (idx, transaction) in data.get_transactions().iter().enumerate() {
                    let cur = Currency::new_float(transaction.1, Some(opt.clone()));
                    println!(
                        "{}. {}, {}: {}",
                        idx + 1,
                        if transaction.0 {
                            bal = bal.add(cur.value() * -1.0);
                            format!("{} {}", "⬇", cur.format()).bright_red().bold()
                        } else {
                            bal = bal.add(cur.value());
                            format!("{} {}", "⬆", cur.format()).bright_green().bold()
                        },
                        "Balance".bright_blue().bold(),
                        bal.format().bright_yellow().bold(),
                    )
                }
            }
        }
    } else {
        print_amount("Balance", bal);
    }
}
