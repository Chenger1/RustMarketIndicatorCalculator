use std::thread;
use std::time::Duration;
use crate::logic::calculate_rsi;
use crate::structs::{Ticker, Symbol};
use crate::exchanges::api_client::ApiClient;
use crate::consts;

pub fn get_symbols <'a>(api_client: &Box<dyn ApiClient>, interval: &String) -> Vec<Symbol>{
    let mut symbols = api_client.get_symbols();
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

fn listen_symbol(symbol: &String, interval: &String, client: &Box<dyn ApiClient>){
    let klines = client.get_klines(symbol, interval, Some(&String::from("10")));
    let rsi = calculate_rsi(klines);
    println!("Symbol {}. Current RSI: {}. Interval: {}", symbol, rsi, interval);
    println!("--------");
}

pub fn listen_symbols(symbols: Vec<Symbol>, client: &Box<dyn ApiClient>){
    let sleep_duration = Duration::from_secs(1);
    loop{
        for symbol in symbols.iter(){
            listen_symbol(&symbol.symbol, &symbol.interval, &client);
        }
        thread::sleep(sleep_duration);
    }
}
