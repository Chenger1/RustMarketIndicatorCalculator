use tokio::task::JoinSet;
use std::sync::Arc;
use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, Database};
use core::trading::TradingService;
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
    for chunk in consts::INTERVALS{
        let db1 = Arc::clone(&db_storage);
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("linear"));
            let interval = &chunk.to_string();
            let service = TradingService::new(
                String::from("Bybit Futures"),
                interval,
                &client,
                db1
            );
            service.run().await;
        });
        let db2 = Arc::clone(&db_storage);
        set.spawn(async move {
            let client = BybitApiClient::new(String::from("spot"));
            let interval = &chunk.to_string();
            let service = TradingService::new(
                String::from("Bybit Spot"),
                interval,
                &client,
                db2
            );
            service.run().await;
        });
        let db3 = Arc::clone(&db_storage);
        set.spawn(async move {
            let client = BinanceFuturesApiClient::new();
            let interval = &chunk.to_string();
            let service = TradingService::new(
                String::from("Binance Futures"),
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
    // TODO: add error handling
    dotenv().ok();
    let _ = start_listening().await;
}
