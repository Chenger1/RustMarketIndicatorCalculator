use tokio::task::JoinSet;
use trading::listener::listen_symbols;
use crate::exchanges::{bybit::BybitApiClient, binance::BinanceFuturesApiClient};
use crate::storage::json::JsonStorage;

mod structs;
mod exchanges;
mod consts;
mod storage;
mod trading;


async fn start_listening(){
    let mut set = JoinSet::new();
    for chunk in consts::INTERVALS{
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("linear"));
            let mut storage = JsonStorage::new("bybit_futures.json");
            listen_symbols(String::from("Bybit Futures"),&chunk.to_string(), &client, &mut storage).await;
        });
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("spot"));
            let mut storage = JsonStorage::new("bybit_spot.json");
            listen_symbols(String::from("Bybit Spot"), &chunk.to_string(), &client, &mut storage).await;
        });
        set.spawn(async move {
            let client = BinanceFuturesApiClient::new();
            let mut storage = JsonStorage::new("binance_futures.json");
            listen_symbols(String::from("Binance Futures"), &chunk.to_string(), &client, &mut storage).await;
        });
    }
    set.join_all().await;
}

#[tokio::main]
async fn main(){
    // TODO: add error handling
    let _ = start_listening().await;
}
