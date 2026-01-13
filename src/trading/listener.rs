use std::time::Duration;
use tokio::time::sleep;
use crate::trading::logic::calculate_rsi;
use crate::structs::{Ticker, Symbol, Indicator};
use crate::exchanges::api_client::ApiClient;
use crate::storage::storage_trait::Storage;
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

async fn listen_symbol(exchange: &String, symbol: &String, interval: &String, client: &impl ApiClient) -> Indicator{
    let klines = client.get_klines(symbol, interval, Some(&String::from("10"))).await;
    let rsi = calculate_rsi(klines);
    println!("Exchange: {exchange}, Symbol {}. Current RSI: {}. Interval: {}", symbol, rsi, interval);
    println!("--------");
    Indicator{
        symbol: symbol.clone(),
        interval: interval.clone(),
        value: rsi
    }
}

pub async fn listen_symbols(exchange: String, interval: &String, client: &impl ApiClient, storage: &mut impl Storage){
    let symbols: Vec<Symbol> = get_symbols(client, interval).await;
    let sleep_duration = Duration::from_secs(3);
    loop{
        let mut indicators: Vec<Indicator> = vec![];
        for symbol in symbols.iter(){
            indicators.push(listen_symbol(&exchange, &symbol.symbol, &symbol.interval, client).await);
        }
        storage.write_data(indicators);
        sleep(sleep_duration).await;
    }
}
