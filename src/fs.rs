use bincode::{config, Decode, Encode};
use currency_rs::{Currency, CurrencyOpts};
use dirs::{config_dir, data_dir};
use serde::Deserialize;

use std::fs;

use super::APP_NAME;

const DATA_FILE_NAME: &'static str = "data";

#[derive(Encode, Decode)]
pub struct FigData {
    current_balance: f64,
    transactions: Vec<(bool, f64)>,
}

impl FigData {
    pub fn get_balance(&self) -> f64 {
        self.current_balance
    }
    pub fn balance(&mut self, new: f64) {
        self.current_balance = new
    }
    pub fn get_transactions(&self) -> &Vec<(bool, f64)> {
        &self.transactions
    }
    pub fn add_transaction(&mut self, add: bool, amt: f64) {
        self.transactions.push((add, amt))
    }
}

#[derive(Deserialize)]
pub struct FigConfig {
    currency: Option<CurrencyData>,
}

impl FigConfig {
    pub fn get_opts(&self) -> CurrencyOpts {
        let mut opt = CurrencyOpts::new();
        if let Some(c) = &self.currency {
            if let Some(x) = &c.symbol {
                opt = opt.set_symbol(x.clone());
            }
            if let Some(x) = &c.separator {
                opt = opt.set_separator(x.clone());
            }
            if let Some(x) = &c.decimal {
                opt = opt.set_decimal(x.clone());
            }
            if let Some(x) = &c.precision {
                opt = opt.set_precision(*x);
            }
            if let Some(x) = &c.pattern {
                opt = opt.set_pattern(x.clone());
            }
            if let Some(x) = &c.negative_pattern {
                opt = opt.set_negative_pattern(x.clone());
            }
            if let Some(x) = &c.rounding {
                opt = opt.set_increment(*x);
            }
            if let Some(x) = &c.vedic {
                opt = opt.set_use_vedic(*x);
            }
        }
        opt
    }
}

#[derive(Deserialize)]
struct CurrencyData {
    symbol: Option<String>,
    separator: Option<String>,
    decimal: Option<String>,
    precision: Option<i64>,
    pattern: Option<String>,
    negative_pattern: Option<String>,
    rounding: Option<f64>,
    vedic: Option<bool>,
}

pub fn set_fs() -> (FigData, FigConfig) {
    let config = config::standard();
    let mut data_dir = data_dir().expect("Can't find the data directory.");
    data_dir.push(APP_NAME);
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect("Can't create data directory")
    }
    let data_file = data_dir.join(DATA_FILE_NAME);
    if !data_file.is_file() {
        let data = FigData {
            current_balance: 0.0,
            transactions: vec![],
        };
        let out = bincode::encode_to_vec(&data, config).expect("Couldn't encode data");
        fs::write(&data_file, out).expect("Couldn't write to disk");
    }
    let (data, _): (FigData, _) = bincode::decode_from_slice(
        &fs::read(&data_file).expect("Couldn't read data from disk")[..],
        config,
    )
    .expect("Couldn't decode data");
    let mut config_dir = config_dir().expect("Can't find the config directory.");
    config_dir.push(APP_NAME);
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Can't create config directory")
    }
    let config_file = config_dir.join(format!("{APP_NAME}.toml"));
    if !config_file.is_file() {
        fs::write(&config_file, "").expect("Couldn't create file")
    }
    let config =
        toml::from_str(&fs::read_to_string(&config_file).expect("Couldn't read config file"))
            .expect("Couldn't decode config file");
    (data, config)
}

pub fn store_data(data: FigData) {
    let mut data_dir = data_dir().expect("Can't find the data directory.");
    data_dir.push(APP_NAME);
    let data_file = data_dir.join(DATA_FILE_NAME);
    let config = config::standard();
    let out = bincode::encode_to_vec(&data, config).expect("Couldn't encode data");
    fs::write(&data_file, out).expect("Couldn't write to disk");
}
