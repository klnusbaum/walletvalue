use std::{env, path};
use std::fs::File;
use serde_yaml;
use error::Result;

pub use self::error::Error;

mod error;

pub type Symbol = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WalletType {
    Ripple,
    Ethereum,
    Generic(Symbol),
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub wallet_type: WalletType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub wallets: Vec<Wallet>
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config> {
        args.next();
        let config_file = match args.next() {
            Some(arg) => path::PathBuf::from(arg),
            None => Config::default_config_path()?,
        };

        let config_file = match File::open(config_file){
           Ok(file) => file,
           Err(ioerr) => return Err(Error::Io(ioerr).into()),
        };

        match serde_yaml::from_reader(config_file) {
            Ok(config) => Ok(config),
            Err(yamlerr) => Err(Error::Yaml(yamlerr).into()),
        }
    }

    fn default_config_path() -> Result<path::PathBuf> {
        let mut home_dir = match env::home_dir() {
            Some(path_buf) => path_buf,
            None => return Err(Error::NoHomeDir.into()),
        };

        home_dir.push(".config");
        home_dir.push("walletvalue");
        home_dir.push("config.yaml");

        Ok(home_dir)
    }
}
