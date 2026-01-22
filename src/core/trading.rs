use crate::core::logic::IndicatorsCalculator;
use crate::entity::symbols::Model as Symbol;
use crate::exchanges::api_client::ApiClient;
use crate::storage::Storage;
use crate::structs::Indicator;
use crate::entity::indicators::{IndicatorType, Interval};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct TradingService<'i, 'a, T, S>
where
    T: ApiClient,
    S: Storage,
{
    exchange: String,
    exchange_id: i32,
    interval: &'i String,
    api_client: &'a T,
    storage: Arc<S>,
}

impl<'i, 'a, T, S> TradingService<'i, 'a, T, S>
where
    T: ApiClient,
    S: Storage,
{
    pub fn new(
        exchange: String,
        exchange_id: i32,
        interval: &'i String,
        api_client: &'a T,
        storage: Arc<S>,
    ) -> Self {
        TradingService {
            exchange,
            exchange_id,
            interval,
            api_client,
            storage,
        }
    }

    async fn listen_symbol(&self, symbol: &Symbol) -> (Indicator, Indicator) {
        let klines = self
            .api_client
            .get_klines(&symbol.tile, self.interval, Some(&String::from("10")))
            .await;
        let rsi = IndicatorsCalculator::calculate_rsi(&klines);
        let price = IndicatorsCalculator::calculate_price(&klines);
        println!(
            "Exchange: {}, Symbol {}. Current RSI: {}. Current Price: {}. Interval: {}",
            self.exchange, symbol.tile, rsi, price, self.interval
        );
        println!("--------");
        (
            Indicator {
                value: rsi,
                symbol_id: symbol.id.clone(),
                indicator_type: IndicatorType::RSI,
                interval: Interval::from_string(self.interval).unwrap()
            },
            Indicator {
                value: price,
                symbol_id: symbol.id.clone(),
                indicator_type: IndicatorType::Price,
                interval: Interval::from_string(self.interval).unwrap()
            },
        )
    }

    pub async fn run(&self) {
        let symbols = self.storage.get_symbols(self.exchange_id).await;
        let sleep_duration = Duration::from_secs(3);
        loop {
            let mut rsi_indicators: Vec<Indicator> = vec![];
            let mut price_indicators: Vec<Indicator> = vec![];
            for symbol in symbols.iter() {
                let (rsi, price) = self.listen_symbol(&symbol).await;
                rsi_indicators.push(rsi);
                price_indicators.push(price);
            }
            self.storage.save_indicators(rsi_indicators, IndicatorType::RSI, Interval::from_string(self.interval).unwrap()).await;
            self.storage.save_indicators(price_indicators, IndicatorType::Price, Interval::from_string(self.interval).unwrap()).await;
            sleep(sleep_duration).await;
        }
    }
}
