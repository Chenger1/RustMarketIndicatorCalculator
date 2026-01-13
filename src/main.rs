use tokio::task::JoinSet;
use core::trading::TradingService;
use crate::exchanges::{bybit::BybitApiClient, binance::BinanceFuturesApiClient};
use crate::storage::json::JsonStorage;

mod structs;
mod exchanges;
mod consts;
mod storage;
mod core;


async fn start_listening(){
    let mut set = JoinSet::new();
    for chunk in consts::INTERVALS{
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("linear"));
            let mut storage = JsonStorage::new("bybit_futures.json");
            let interval = &chunk.to_string();
            let service = TradingService::new(
                String::from("Bybit Futures"),
                interval,
                &client
            );
            service.run(&mut storage).await;
        });
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("spot"));
            let mut storage = JsonStorage::new("bybit_spot.json");
            let interval = &chunk.to_string();
            let service = TradingService::new(
                String::from("Bybit Spot"),
                interval,
                &client
            );
            service.run(&mut storage).await;
        });
        set.spawn(async move {
            let client = BinanceFuturesApiClient::new();
            let mut storage = JsonStorage::new("binance_futures.json");
            let interval = &chunk.to_string();
            let service = TradingService::new(
                String::from("Binance Futures"),
                interval,
                &client
            );
            service.run(&mut storage).await;
        });
    }
    set.join_all().await;
}

#[tokio::main]
async fn main(){
    // TODO: add error handling
    let _ = start_listening().await;
}
