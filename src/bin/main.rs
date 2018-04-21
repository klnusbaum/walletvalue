extern crate walletvalue;

use std::{env, process};
use walletvalue::config::Config;
use walletvalue::fetcher::{WalletAggregator, RippleFetcher, EtheriumFetcher, GenericFetcher};
use walletvalue::Result;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error initializing config: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Error fetching wallets: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    let aggregator = WalletAggregator::new(
        RippleFetcher::new(),
        EtheriumFetcher::new(),
        GenericFetcher::new()
    );
    aggregator.fetch_wallets(config.wallets);
    Ok(())
}
