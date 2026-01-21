use crate::exchanges::api_client::ApiClient;
use crate::exchanges::{binance::BinanceFuturesApiClient, bybit::BybitApiClient};
use crate::storage::Storage;
use crate::storage::sql::DBStorage;
use std::collections::HashMap;
use std::sync::Arc;

pub const BYBIT_FUTURES_NAME: &str = "Bybit Futures";
pub const BINANCE_FUTURES_NAME: &str = "Binance Futures";
pub const BYBIT_SPOT_NAME: &str = "Bybit Spot";

pub struct InitData {
    db: Arc<DBStorage>,
}

impl InitData {
    pub fn new(db: Arc<DBStorage>) -> Self {
        InitData { db }
    }

    async fn init_exchange(&self, exchange: String, api_client: impl ApiClient) -> i32 {
        let exchange_id: i32 = self.db.create_exchange(&exchange).await;
        self.db
            .create_symbols(api_client.get_symbols().await, exchange_id)
            .await;
        exchange_id
    }

    pub async fn init(&self) -> HashMap<String, i32> {
        let mut ids: HashMap<String, i32> = HashMap::new();
        ids.insert(
            BYBIT_FUTURES_NAME.to_string(),
            self.init_exchange(
                BYBIT_FUTURES_NAME.to_string(),
                BybitApiClient::new(String::from("linear")),
            )
            .await,
        );
        ids.insert(
            BYBIT_SPOT_NAME.to_string(),
            self.init_exchange(
                BYBIT_SPOT_NAME.to_string(),
                BybitApiClient::new(String::from("spot")),
            )
            .await,
        );
        ids.insert(
            BINANCE_FUTURES_NAME.to_string(),
            self.init_exchange(
                BINANCE_FUTURES_NAME.to_string(),
                BinanceFuturesApiClient::new(),
            )
            .await,
        );
        ids
    }
}
