use tokio::task::JoinSet;
use listener::listen_symbols;
use crate::exchanges::{bybit::BybitApiClient, binance::BinanceFuturesApiClient};

mod api_client;
mod structs;
mod logic;
mod utils;
mod listener;
mod exchanges;
mod consts;


async fn start_listening(){
    let mut set = JoinSet::new();
    for chunk in consts::INTERVALS{
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("linear"));
            listen_symbols(&chunk.to_string(), &client).await;
        });
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("spot"));
            listen_symbols(&chunk.to_string(), &client).await;
        });
        set.spawn(async move {
            let client = BinanceFuturesApiClient::new();
            listen_symbols(&chunk.to_string(), &client).await;
        });
    }
    set.join_all().await;
}

#[tokio::main]
async fn main(){
    // TODO: add error handling
    let _ = start_listening().await;
}
