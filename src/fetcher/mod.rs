use config;
use config::WalletType;
use error::Result;

pub use self::error::Error;

mod error;

pub type WalletResults = Vec<Result<WalletValue>>;

pub struct WalletValue {
    pub symbol: String,
    pub fiat_symbol: String,
    pub amount: f64, // TODO DON'T REPRESENT CURRENCY AS FLOATING POINT NUMBERS
    pub fiat_amount: f64,
}

pub struct WalletAggregator {
    ripple_fetcher: RippleFetcher,
    ethereum_fetcher: EtheriumFetcher,
    generic_fetcher: GenericFetcher,
    // value_printer: ValuePrinter,
}

impl WalletAggregator {
    pub fn new(
        ripple_fetcher: RippleFetcher,
        ethereum_fetcher: EtheriumFetcher,
        generic_fetcher: GenericFetcher,
    ) -> WalletAggregator {
        WalletAggregator {
            ripple_fetcher,
            ethereum_fetcher,
            generic_fetcher,
        }
    }

    pub fn fetch_wallets(&self, wallets: Vec<config::Wallet>) -> WalletResults {
        wallets.iter().map(|wallet| self.fetch_wallet(wallet)).collect()
    }

    fn fetch_wallet(&self, wallet: &config::Wallet) -> Result<WalletValue> {
        match wallet.wallet_type {
            WalletType::Ripple => self.ripple_fetcher.get_wallet_value(wallet),
            WalletType::Ethereum => self.ethereum_fetcher.get_wallet_value(wallet),
            WalletType::Generic(_) => self.generic_fetcher.get_wallet_value(wallet),
        }
    }
}

pub struct RippleFetcher {}

impl RippleFetcher {
    pub fn new() -> RippleFetcher {
        RippleFetcher {}
    }

    pub fn get_wallet_value(&self, wallet: &config::Wallet) -> Result<WalletValue> {
        unimplemented!()
    }
}

pub struct EtheriumFetcher {}

impl EtheriumFetcher {
    pub fn new() -> EtheriumFetcher {
        EtheriumFetcher {}
    }

    pub fn get_wallet_value(&self, wallet: &config::Wallet) -> Result<WalletValue> {
        unimplemented!()
    }
}

pub struct GenericFetcher {}

impl GenericFetcher {
    pub fn new() -> GenericFetcher {
        GenericFetcher {}
    }

    pub fn get_wallet_value(&self, wallet: &config::Wallet) -> Result<WalletValue> {
        unimplemented!()
    }
}
