use std::time::Duration;
use tokio::time::sleep;
use crate::core::logic::Indicators;
use crate::exchanges::api_client::ApiClient;
use crate::structs::{Symbol, Indicator};
use crate::storage::Storage;

pub struct TradingService<'i, 'a, T>
where 
    T: ApiClient
{
    exchange: String,
    interval: &'i String,
    api_client: &'a T
}

impl<'i, 'a, T> TradingService<'i, 'a, T>
where 
    T: ApiClient
{
    pub fn new(exchange: String, interval: &'i String, api_client: &'a T) -> Self{
        TradingService{
            exchange: exchange,
            interval: interval,
            api_client: api_client
        }
    }

    async fn listen_symbol(&self, symbol: &String) -> Indicator{
        let klines = self.api_client.get_klines(symbol, self.interval, Some(&String::from("10"))).await;
        let rsi = Indicators::calculate_rsi(klines);
        println!("Exchange: {}, Symbol {}. Current RSI: {}. Interval: {}", self.exchange, symbol, rsi, self.interval);
        println!("--------");
        Indicator{
            symbol: symbol.clone(),
            interval: self.interval.clone(),
            value: rsi
        }
    }

    pub async fn run(&self, storage: &mut impl Storage){
        let symbols: Vec<Symbol> = self.api_client.get_symbols().await;
        let sleep_duration = Duration::from_secs(3);
        loop{
            let mut indicators: Vec<Indicator> = vec![];
            for symbol in symbols.iter(){
                indicators.push(self.listen_symbol(&symbol.symbol).await);
            }
            storage.write_data(indicators).await;
            sleep(sleep_duration).await;
        }
    }
}
