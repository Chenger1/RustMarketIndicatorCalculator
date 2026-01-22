use tokio::task::JoinSet;
use std::sync::Arc;
use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, Database};
use core::trading::TradingService;
use core::data::{InitData, BYBIT_FUTURES_NAME, BINANCE_FUTURES_NAME, BYBIT_SPOT_NAME};
use crate::exchanges::{bybit::BybitApiClient, binance::BinanceFuturesApiClient};
use crate::storage::sql::DBStorage;

mod structs;
mod exchanges;
mod consts;
mod storage;
mod core;
mod entity;

async fn start_listening() -> Result<(), Box<dyn std::error::Error>>{
    let mut set = JoinSet::new();
    let db: DatabaseConnection = Database::connect(std::env::var("DATABASE_URL")?.as_str()).await?;
    let db_storage = Arc::new(DBStorage::new(db));
    let init_data = InitData::new(Arc::clone(&db_storage));
    let exchanges_ids = Arc::new(init_data.init().await);
    for chunk in consts::INTERVALS{
        let db1 = Arc::clone(&db_storage);
        let exchanges_ids1 = Arc::clone(&exchanges_ids);
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("linear"));
            let interval = &chunk.to_string();
            let service = TradingService::new(
                BYBIT_FUTURES_NAME.to_string(),
                exchanges_ids1.get(BYBIT_FUTURES_NAME).unwrap().to_owned(),
                interval,
                &client,
                db1
            );
            service.run().await;
        });
        let db2 = Arc::clone(&db_storage);
        let exchanges_ids2 = Arc::clone(&exchanges_ids);
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("spot"));
            let interval = &chunk.to_string();
            let service = TradingService::new(
                BYBIT_SPOT_NAME.to_string(),
                exchanges_ids2.get(BYBIT_SPOT_NAME).unwrap().to_owned(),
                interval,
                &client,
                db2
            );
            service.run().await;
        });
        let db3 = Arc::clone(&db_storage);
        let exchanges_ids3 = Arc::clone(&exchanges_ids);
        set.spawn(async move {
            let client = BinanceFuturesApiClient::new();
            let interval = &chunk.to_string();
            let service = TradingService::new(
                BINANCE_FUTURES_NAME.to_string(),
                exchanges_ids3.get(BINANCE_FUTURES_NAME).unwrap().to_owned(),
                interval,
                &client,
                db3
            );
            service.run().await;
        });
    }
    set.join_all().await;
    Ok(())
}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let _ = start_listening().await;
}
