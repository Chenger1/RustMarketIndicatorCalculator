use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use crate::core::logic::Indicators;
use crate::exchanges::api_client::ApiClient;
use crate::structs::Indicator;
use crate::storage::Storage;
use crate::entity::symbols::Model as Symbol;

pub struct TradingService<'i, 'a, T, S>
where 
    T: ApiClient,
    S: Storage
{
    exchange: String,
    interval: &'i String,
    api_client: &'a T,
    storage: Arc<S>
}

impl<'i, 'a, T, S> TradingService<'i, 'a, T, S>
where 
    T: ApiClient,
    S: Storage
{
    pub fn new(exchange: String, interval: &'i String, api_client: &'a T, storage: Arc<S>) -> Self{
        TradingService{
            exchange: exchange,
            interval: interval,
            api_client: api_client,
            storage: storage
        }
    }

    async fn listen_symbol(&self, symbol: &Symbol) -> Indicator{
        let klines = self.api_client.get_klines(&symbol.tile, self.interval, Some(&String::from("10"))).await;
        let rsi = Indicators::calculate_rsi(klines);
        println!("Exchange: {}, Symbol {}. Current RSI: {}. Interval: {}", self.exchange, symbol.tile, rsi, self.interval);
        println!("--------");
        Indicator{
            value: rsi,
            symbol_id: symbol.id.clone(),
        }
    }

    pub async fn run(&self){
        let exchange_id = self.storage.create_exchange(&self.exchange).await;
        let symbols = self.storage.create_symbols(self.api_client.get_symbols().await, exchange_id).await;
        let sleep_duration = Duration::from_secs(3);
        loop{
            let mut indicators: Vec<Indicator> = vec![];
            for symbol in symbols.iter(){
                indicators.push(self.listen_symbol(&symbol).await);
            }
            self.storage.save_indicators(indicators).await;
            sleep(sleep_duration).await;
        }
    }
}
