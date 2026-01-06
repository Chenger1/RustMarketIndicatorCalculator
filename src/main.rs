use std::thread;
use listener::{get_symbols, listen_symbols};
use crate::exchanges::get_exchange_client;
use crate::exchanges::api_client::ExchangeEnum;

mod api_client;
mod structs;
mod logic;
mod utils;
mod listener;
mod exchanges;
mod consts;


fn start_listening(){
    let exchanges: [ExchangeEnum; 3] = [
        ExchangeEnum::BybitFutures,
        ExchangeEnum::BybitSpot,
        ExchangeEnum::BinanceFutures
    ];

    thread::scope(|s| {
        for exchange in exchanges.iter(){
            for chunk in consts::INTERVALS{
                s.spawn(|| {
                    let client = get_exchange_client(exchange);
                    let tickers = get_symbols(&client, &chunk.to_string());
                    listen_symbols(tickers, &client);
                });
            }
        }
    })
}

fn main(){
    // TODO: add error handling
    let _ = start_listening();
}
