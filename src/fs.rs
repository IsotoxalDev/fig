use currency_rs_isotoxal::CurrencyOpts;
use serde::{Deserialize, Serialize};

use std::{env, fs, path::PathBuf};

use super::APP_NAME;

const DATA_FILE_NAME: &'static str = "data";

#[derive(Serialize, Deserialize)]
pub struct FigData {
    current_balance: f64,
    sub: Vec<bool>,
    amt: Vec<f64>,
}

impl FigData {
    pub fn get_balance(&self) -> f64 {
        self.current_balance
    }
    pub fn balance(&mut self, new: f64) {
        self.current_balance = new
    }
    pub fn get_transactions(&self) -> Vec<(&bool, &f64)> {
        self.sub.iter().zip(self.amt.iter()).collect()
    }
    pub fn add_transaction(&mut self, sub: bool, amt: f64) {
        self.sub.push(sub);
        self.amt.push(amt);
    }
}

#[derive(Deserialize, Clone, Copy)]
pub enum FigSaveType {
    Xml,
    Bin,
}

impl Default for FigSaveType {
    fn default() -> Self {
        Self::Xml
    }
}

#[derive(Deserialize)]
pub struct FigConfig {
    add_char: Option<String>,
    take_char: Option<String>,
    save_type: Option<FigSaveType>,
    currency: Option<CurrencyData>,
}

impl FigConfig {
    pub fn save_type(&self) -> FigSaveType {
        self.save_type.clone().unwrap_or_default()
    }
    pub fn get_character(&self) -> (String, String) {
        (
            if let Some(c) = &self.add_char {
                c.clone()
            } else {
                "⬆".into()
            },
            if let Some(c) = &self.take_char {
                c.clone()
            } else {
                "⬇".into()
            },
        )
    }
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

fn get_data_path() -> PathBuf {
    let mut out = if cfg!(windows) {
        PathBuf::from(env::var("APPDATA").expect("Couldn't find the environment var APPDATA"))
    } else {
        match env::var("XDG_DATA_HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => PathBuf::from(
                env::var("HOME").expect("Couldn't find the environment var HOME or XDG_DATA_HOME"),
            )
            .join(".local/share/"),
        }
    };
    out.push(APP_NAME);
    out
}

fn get_config_path() -> PathBuf {
    let mut out = if cfg!(windows) {
        PathBuf::from(env::var("APPDATA").expect("Couldn't find the environment var APPDATA"))
    } else {
        match env::var("XDG_CONFIG_HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => PathBuf::from(
                env::var("HOME")
                    .expect("Couldn't find the environment var HOME or XDG_CONFIG_HOME"),
            )
            .join(".config"),
        }
    };
    out.push(APP_NAME);
    out
}

pub fn get_data(save_type: FigSaveType) -> FigData {
    let mut file_name = get_data_path().join(DATA_FILE_NAME);
    match save_type {
        FigSaveType::Bin => {
            bincode::deserialize(&fs::read(file_name).expect("Couldn't read data from disk"))
                .expect("Couldn't decode data")
        }
        FigSaveType::Xml => {
            file_name.set_extension("xml");
            match serde_xml_rs::from_str(
                &fs::read_to_string(file_name).expect("Couldn't read data from disk"),
            ) {
                Ok(x) => x,
                Err(serde_xml_rs::Error::Custom { field }) => {
                    if field == "missing field `sub`" || field == "missing field `amt`" {
                        FigData {
                            current_balance: 0.0,
                            sub: vec![],
                            amt: vec![],
                        }
                    } else {
                        panic!("Couldn't decode data")
                    }
                }
                Err(_) => panic!("Couldn't decode data"),
            }
        }
    }
}

pub fn store_data(data: FigData, save_type: FigSaveType) {
    let mut file_name = get_data_path().join(DATA_FILE_NAME);
    match save_type {
        FigSaveType::Bin => fs::write(
            file_name,
            bincode::serialize(&data).expect("Couldn't encode data"),
        )
        .expect("Couldn't write to disk"),
        FigSaveType::Xml => {
            file_name.set_extension("xml");
            fs::write(
                file_name,
                serde_xml_rs::to_string(&data).expect("Couldn't encode data"),
            )
            .expect("Couldn't write data to disk")
        }
    }
}

pub fn set_fs() -> (FigData, FigConfig) {
    let config_dir = get_config_path();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Can't create config directory")
    }
    let config_file = config_dir.join(format!("{APP_NAME}.toml"));
    if !config_file.is_file() {
        fs::write(&config_file, "").expect("Couldn't create file")
    }
    let config: FigConfig =
        toml::from_str(&fs::read_to_string(&config_file).expect("Couldn't read config file"))
            .expect("Couldn't decode config file");
    let data_dir = get_data_path();
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect("Can't create data directory")
    }
    let file_type: FigSaveType = config.save_type();
    let mut data_file = data_dir.join(DATA_FILE_NAME);
    if let FigSaveType::Xml = &file_type {
        data_file.set_extension("xml");
    }
    if !data_file.is_file() {
        let data = FigData {
            current_balance: 0.0,
            sub: vec![],
            amt: vec![],
        };
        store_data(data, file_type)
    }
    let data: FigData = get_data(file_type);
    (data, config)
}
