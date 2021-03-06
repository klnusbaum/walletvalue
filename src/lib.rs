extern crate reqwest;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

pub use error::{Error, Result};
pub mod config;
pub mod fetcher;

mod error;
