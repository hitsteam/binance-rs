use crate::account::Account;
use crate::client::Client;
use crate::config::Config;
use crate::futures::account::FuturesAccount;
use crate::futures::general::FuturesGeneral;
use crate::futures::market::FuturesMarket;
use crate::futures::coinaccount::FuturesCoinAccount;
use crate::futures::coingeneral::FuturesCoinGeneral;
use crate::futures::coinmarket::FuturesCoinMarket;
use crate::futures::userstream::FuturesUserStream;
use crate::general::General;
use crate::market::Market;
use crate::userstream::UserStream;
use crate::savings::Savings;

#[allow(clippy::all)]
pub enum API {
    Spot(Spot),
    Savings(Sapi),
    Futures(Futures),
    FuturesCoin(FuturesCoin)
}

/// Endpoint for production and test orders.
///
/// Orders issued to test are validated, but not sent into the matching engine.
pub enum Spot {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    AvgPrice,
    Ticker24hr,
    Price,
    BookTicker,
    Order,
    OrderTest,
    OpenOrders,
    AllOrders,
    Oco,
    OrderList,
    AllOrderList,
    OpenOrderList,
    Account,
    MyTrades,
    UserDataStream,
}

pub enum Sapi {
    AllCoins,
    AssetDetail,
    DepositAddress,
    SpotFuturesTransfer,
}

pub enum Futures {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    ContinuousKlines,
    IndexPriceKlines,
    MarkPriceKlines,
    PremiumIndex,
    FundingRate,
    Ticker24hr,
    TickerPrice,
    BookTicker,
    AllForceOrders,
    AllOpenOrders,
    AllOrders,
    UserTrades,
    Order,
    PositionRisk,
    Balance,
    PositionSide,
    OpenInterest,
    OpenInterestHist,
    TopLongShortAccountRatio,
    TopLongShortPositionRatio,
    GlobalLongShortAccountRatio,
    TakerlongshortRatio,
    LvtKlines,
    IndexInfo,
    ChangeInitialLeverage,
    Account,
    OpenOrders,
    UserDataStream,
    Income,
    MarginType,
    SubAccountSummary
}

pub enum FuturesCoin {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    ContinuousKlines,
    IndexPriceKlines,
    MarkPriceKlines,
    PremiumIndex,
    FundingRate,
    Ticker24hr,
    TickerPrice,
    BookTicker,
    AllForceOrders,
    AllOpenOrders,
    AllOrders,
    UserTrades,
    Order,
    PositionRisk,
    Balance,
    PositionSide,
    OpenInterest,
    OpenInterestHist,
    TopLongShortAccountRatio,
    TopLongShortPositionRatio,
    GlobalLongShortAccountRatio,
    TakerlongshortRatio,
    LvtKlines,
    IndexInfo,
    ChangeInitialLeverage,
    Account,
    OpenOrders,
    UserDataStream,
    Income,
    MarginType,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(route) => match route {
                Spot::Ping => "/api/v3/ping",
                Spot::Time => "/api/v3/time",
                Spot::ExchangeInfo => "/api/v3/exchangeInfo",
                Spot::Depth => "/api/v3/depth",
                Spot::Trades => "/api/v3/trades",
                Spot::HistoricalTrades => "/api/v3/historicalTrades",
                Spot::AggTrades => "/api/v3/aggTrades",
                Spot::Klines => "/api/v3/klines",
                Spot::AvgPrice => "/api/v3/avgPrice",
                Spot::Ticker24hr => "/api/v3/ticker/24hr",
                Spot::Price => "/api/v3/ticker/price",
                Spot::BookTicker => "/api/v3/ticker/bookTicker",
                Spot::Order => "/api/v3/order",
                Spot::OrderTest => "/api/v3/order/test",
                Spot::OpenOrders => "/api/v3/openOrders",
                Spot::AllOrders => "/api/v3/allOrders",
                Spot::Oco => "/api/v3/order/oco",
                Spot::OrderList => "/api/v3/orderList",
                Spot::AllOrderList => "/api/v3/allOrderList",
                Spot::OpenOrderList => "/api/v3/openOrderList",
                Spot::Account => "/api/v3/account",
                Spot::MyTrades => "/api/v3/myTrades",
                Spot::UserDataStream => "/api/v3/userDataStream",
            },
            API::Savings(route) => match route {
                Sapi::AllCoins => "/sapi/v1/capital/config/getall",
                Sapi::AssetDetail => "/sapi/v1/asset/assetDetail",
                Sapi::DepositAddress => "/sapi/v1/capital/deposit/address",
                Sapi::SpotFuturesTransfer => "/sapi/v1/futures/transfer",
            },
            API::Futures(route) => match route {
                Futures::Ping => "/fapi/v1/ping",
                Futures::Time => "/fapi/v1/time",
                Futures::ExchangeInfo => "/fapi/v1/exchangeInfo",
                Futures::Depth => "/fapi/v1/depth",
                Futures::Trades => "/fapi/v1/trades",
                Futures::HistoricalTrades => "/fapi/v1/historicalTrades",
                Futures::AggTrades => "/fapi/v1/aggTrades",
                Futures::Klines => "/fapi/v1/klines",
                Futures::ContinuousKlines => "/fapi/v1/continuousKlines",
                Futures::IndexPriceKlines => "/fapi/v1/indexPriceKlines",
                Futures::MarkPriceKlines => "/fapi/v1/markPriceKlines",
                Futures::PremiumIndex => "/fapi/v1/premiumIndex",
                Futures::FundingRate => "/fapi/v1/fundingRate",
                Futures::Ticker24hr => "/fapi/v1/ticker/24hr",
                Futures::TickerPrice => "/fapi/v2/ticker/price",
                Futures::BookTicker => "/fapi/v1/ticker/bookTicker",
                Futures::AllForceOrders => "/fapi/v1/allForceOrders",
                Futures::AllOpenOrders => "/fapi/v1/allOpenOrders",
                Futures::AllOrders => "/fapi/v1/allOrders",
                Futures::UserTrades => "/fapi/v1/userTrades",
                Futures::PositionSide => "/fapi/v1/positionSide/dual",
                Futures::Order => "/fapi/v1/order",
                Futures::PositionRisk => "/fapi/v2/positionRisk",
                Futures::Balance => "/fapi/v2/balance",
                Futures::OpenInterest => "/fapi/v1/openInterest",
                Futures::OpenInterestHist => "/futures/data/openInterestHist",
                Futures::TopLongShortAccountRatio => "/futures/data/topLongShortAccountRatio",
                Futures::TopLongShortPositionRatio => "/futures/data/topLongShortPositionRatio",
                Futures::GlobalLongShortAccountRatio => "/futures/data/globalLongShortAccountRatio",
                Futures::TakerlongshortRatio => "/futures/data/takerlongshortRatio",
                Futures::LvtKlines => "/fapi/v1/lvtKlines",
                Futures::IndexInfo => "/fapi/v1/indexInfo",
                Futures::ChangeInitialLeverage => "/fapi/v1/leverage",
                Futures::Account => "/fapi/v2/account",
                Futures::OpenOrders => "/fapi/v1/openOrders",
                Futures::UserDataStream => "/fapi/v1/listenKey",
                Futures::Income => "/fapi/v1/income",
                Futures::MarginType => "/fapi/v1/marginType",
                Futures::SubAccountSummary => "/sapi/v2/sub-account/futures/accountSummary",
            },
            API::FuturesCoin(route) => match route {
                FuturesCoin::Ping => "/dapi/v1/ping",
                FuturesCoin::Time => "/dapi/v1/time",
                FuturesCoin::ExchangeInfo => "/dapi/v1/exchangeInfo",
                FuturesCoin::Depth => "/dapi/v1/depth",
                FuturesCoin::Trades => "/dapi/v1/trades",
                FuturesCoin::HistoricalTrades => "/dapi/v1/historicalTrades",
                FuturesCoin::AggTrades => "/dapi/v1/aggTrades",
                FuturesCoin::Klines => "/dapi/v1/klines",
                FuturesCoin::ContinuousKlines => "/dapi/v1/continuousKlines",
                FuturesCoin::IndexPriceKlines => "/dapi/v1/indexPriceKlines",
                FuturesCoin::MarkPriceKlines => "/dapi/v1/markPriceKlines",
                FuturesCoin::PremiumIndex => "/dapi/v1/premiumIndex",
                FuturesCoin::FundingRate => "/dapi/v1/fundingRate",
                FuturesCoin::Ticker24hr => "/dapi/v1/ticker/24hr",
                FuturesCoin::TickerPrice => "/dapi/v2/ticker/price",
                FuturesCoin::BookTicker => "/dapi/v1/ticker/bookTicker",
                FuturesCoin::AllForceOrders => "/dapi/v1/allForceOrders",
                FuturesCoin::AllOpenOrders => "/dapi/v1/allOpenOrders",
                FuturesCoin::AllOrders => "/dapi/v1/allOrders",
                FuturesCoin::UserTrades => "/dapi/v1/userTrades",
                FuturesCoin::PositionSide => "/dapi/v1/positionSide/dual",
                FuturesCoin::Order => "/dapi/v1/order",
                FuturesCoin::PositionRisk => "/dapi/v2/positionRisk",
                FuturesCoin::Balance => "/dapi/v2/balance",
                FuturesCoin::OpenInterest => "/dapi/v1/openInterest",
                FuturesCoin::OpenInterestHist => "/futures/data/openInterestHist",
                FuturesCoin::TopLongShortAccountRatio => "/futures/data/topLongShortAccountRatio",
                FuturesCoin::TopLongShortPositionRatio => "/futures/data/topLongShortPositionRatio",
                FuturesCoin::GlobalLongShortAccountRatio => "/futures/data/globalLongShortAccountRatio",
                FuturesCoin::TakerlongshortRatio => "/futures/data/takerlongshortRatio",
                FuturesCoin::LvtKlines => "/dapi/v1/lvtKlines",
                FuturesCoin::IndexInfo => "/dapi/v1/indexInfo",
                FuturesCoin::ChangeInitialLeverage => "/dapi/v1/leverage",
                FuturesCoin::Account => "/dapi/v2/account",
                FuturesCoin::OpenOrders => "/dapi/v1/openOrders",
                FuturesCoin::UserDataStream => "/dapi/v1/listenKey",
                FuturesCoin::Income => "/dapi/v1/income",
                FuturesCoin::MarginType => "/dapi/v1/marginType",
            },
        })
    }
}

pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;
    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self;
}

impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Savings {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Market {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> UserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesGeneral {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
        }
    }
}


impl Binance for FuturesCoinGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesCoinGeneral {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesCoinGeneral {
        FuturesCoinGeneral {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_coin_rest_api_endpoint.clone(),
            ),
        }
    }
}

impl Binance for FuturesMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesMarket {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for FuturesCoinMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesCoinMarket {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesCoinMarket {
        FuturesCoinMarket {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_coin_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for FuturesAccount {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for FuturesCoinAccount {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_coin_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for FuturesUserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesUserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesUserStream {
        FuturesUserStream {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}
