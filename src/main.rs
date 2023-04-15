use clap::Parser;
use colored::*;
use currency_rs_isotoxal::Currency;
use rustyline::DefaultEditor;

mod cli;
mod fs;

use cli::{FigArgs, FigCommands};
use fs::{FigData, FigSaveType};

const APP_NAME: &'static str = "fig";

fn print_data(text: &str, bal: String) {
    println!(
        "{}: {}",
        text.bright_blue().bold(),
        bal.bright_yellow().bold()
    )
}

fn get_data(
    of: &str,
    amt: Option<f64>,
    msg: Option<String>,
    r: &mut DefaultEditor,
) -> (f64, Option<String>) {
    if let Some(a) = amt {
        (a, msg)
    } else {
        let val = loop {
            if let Ok(inp) = r.readline(&format!(
                "{}: ",
                format!("Enter amount to {}", of).bright_green().bold()
            )) {
                if let Ok(val) = inp.parse::<f64>() {
                    break val;
                }
            }
        };
        let msg = loop {
            if let Ok(inp) =
                r.readline(&format!("{}", "Give a message to this transaction (Y/n): "))
            {
                let inp = inp.to_uppercase();
                if inp == "Y" || inp == "" {
                    break loop {
                        if let Ok(inp) =
                            r.readline(&format!("{}: ", "Enter message".bright_green().bold()))
                        {
                            break Some(inp);
                        }
                    };
                } else if inp == "N" {
                    break None;
                }
            }
        };
        println!("");
        (val, msg)
    }
}

fn set_balance(
    mut bal: Currency,
    amt: Currency,
    mut data: FigData,
    text: &str,
    sub: bool,
    msg: Option<String>,
    save_type: FigSaveType,
) {
    if amt.value() == 0.0 {
        print_data(text, amt.format());
        print_data("Balance", bal.format());
        if let Some(m) = msg {
            print_data("Message", m);
        }
        return;
    }
    bal = bal.add(amt.value() * if sub { -1.0 } else { 1.0 });
    data.balance(bal.value());
    data.add_transaction(sub, amt.value(), msg.clone());
    fs::store_data(data, save_type);
    print_data(text, amt.format());
    print_data("Balance", bal.format());
    if let Some(m) = msg {
        print_data("Message", m);
    }
    return;
}

fn main() {
    let mut r = DefaultEditor::new().unwrap();
    let (data, config) = fs::set_fs();
    let args = FigArgs::parse();
    let opt = config.get_opts();
    let bal = Currency::new_float(data.get_balance(), Some(opt.clone()));
    let save_type = config.save_type();
    if let Some(command) = args.command {
        match command {
            FigCommands::Add { amount, message } => {
                let input_data = get_data("add", amount, message, &mut r);
                let amt = Currency::new_float(input_data.0, Some(opt.clone()));
                set_balance(bal, amt, data, "Adding", false, input_data.1, save_type)
            }
            FigCommands::Take { amount, message } => {
                let input_data = get_data("take", amount, message, &mut r);
                let amt = Currency::new_float(input_data.0, Some(opt.clone()));
                set_balance(bal, amt, data, "Taking", true, input_data.1, save_type)
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
                    let char = config.get_character();

                    println!(
                        "{}. {}, {}: {}{}",
                        idx + 1,
                        if transaction.0 {
                            bal = bal.add(cur.value() * -1.0);
                            format!("{} {}", char.1, cur.format()).bright_red().bold()
                        } else {
                            bal = bal.add(cur.value());
                            format!("{} {}", char.0, cur.format()).bright_green().bold()
                        },
                        "Balance".bright_blue().bold(),
                        bal.format().bright_yellow().bold(),
                        if let Some(m) = transaction.2 {
                            if m == "" {
                                m.clone()
                            } else {
                                format!(
                                    ", {}: {}",
                                    "Message".bright_blue().bold(),
                                    m.bright_yellow().bold()
                                )
                            }
                        } else {
                            "".into()
                        }
                    )
                }
            }
        }
    } else {
        print_data("Balance", bal.format());
    }
}
