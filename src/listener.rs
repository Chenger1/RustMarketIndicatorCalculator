use std::time::Duration;
use tokio::time::sleep;
use crate::logic::calculate_rsi;
use crate::structs::{Ticker, Symbol};
use crate::exchanges::api_client::ApiClient;
use crate::consts;

pub async fn get_symbols <'a>(api_client: &impl ApiClient, interval: &String) -> Vec<Symbol>{
    let mut symbols = api_client.get_symbols().await;
    symbols.sort_by(|a, b| a.volume.partial_cmp(&b.volume).unwrap());
    symbols.reverse();
    let active_symbols: Vec<Ticker> = symbols[0..consts::NUMBER_OF_SYMBOLS].to_vec();
    let mut symbols_with_intervals: Vec<Symbol> = Vec::new();
    for ticker in active_symbols{
        symbols_with_intervals.push(
            Symbol { symbol: ticker.symbol.clone(), interval: interval.clone() }
        )
    }
    symbols_with_intervals
}

async fn listen_symbol(symbol: &String, interval: &String, client: &impl ApiClient){
    let klines = client.get_klines(symbol, interval, Some(&String::from("10"))).await;
    let rsi = calculate_rsi(klines);
    println!("Symbol {}. Current RSI: {}. Interval: {}", symbol, rsi, interval);
    println!("--------");
}

pub async fn listen_symbols(interval: &String, client: &impl ApiClient){
    let symbols: Vec<Symbol> = get_symbols(client, interval).await;
    let sleep_duration = Duration::from_secs(1);
    loop{
        for symbol in symbols.iter(){
            listen_symbol(&symbol.symbol, &symbol.interval, client).await;
        }
        sleep(sleep_duration).await;
    }
}
