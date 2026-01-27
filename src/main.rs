use tokio::task::JoinSet;
use std::sync::Arc;
use std::collections::HashMap;
use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, Database};
use core::trading::TradingService;
use core::data::{InitData, BYBIT_FUTURES_NAME, BINANCE_FUTURES_NAME, BYBIT_SPOT_NAME};
use crate::exchanges::{bybit::BybitApiClient, binance::BinanceFuturesApiClient, api_client::ApiClient};
use crate::storage::sql::DBStorage;

mod structs;
mod exchanges;
mod consts;
mod storage;
mod core;
mod entity;

#[derive(Debug, Clone, Copy)]
enum Exchange{
    BybitFutures,
    BybitSpot,
    Binance
}

impl Exchange{
    fn get_name(&self) -> &'static str{
        match self{
            Exchange::BybitFutures => BYBIT_FUTURES_NAME,
            Exchange::BybitSpot => BYBIT_SPOT_NAME,
            Exchange::Binance => BINANCE_FUTURES_NAME
        }
    }

    fn get_client(&self) -> Arc<dyn ApiClient>{
        match self{
            Exchange::BybitFutures => Arc::new(BybitApiClient::new(String::from("linear"))),
            Exchange::BybitSpot => Arc::new(BybitApiClient::new(String::from("spot"))),
            Exchange::Binance => Arc::new(BinanceFuturesApiClient::new())
        }
    }
}

async fn start_listening() -> Result<(), Box<dyn std::error::Error>>{
    // Setup
    let mut set = JoinSet::new();
    let db: DatabaseConnection = Database::connect(std::env::var("DATABASE_URL")?.as_str()).await?;
    let db_storage = Arc::new(DBStorage::new(db));
    let init_data = InitData::new(Arc::clone(&db_storage));

    // Spawn tasks
    let exchanges_ids = Arc::new(init_data.init().await);
    let exchanges = vec![Exchange::BybitFutures, Exchange::BybitSpot, Exchange::Binance];
    for exchange in exchanges{
        let client = exchange.get_client();

        for interval in consts::INTERVALS{
            spawn_service(&mut set, Arc::clone(&db_storage), Arc::clone(&client), Arc::clone(&exchanges_ids), interval.to_string(), exchange.get_name());
        }
    }
    set.join_all().await;
    Ok(())
}

fn spawn_service(
    set: &mut JoinSet<()>,
    db_storage: Arc<DBStorage>,
    client: Arc<dyn ApiClient>,
    exchanges_ids: Arc<HashMap<String, i32>>,
    interval: String,
    exchange_name: &str
){
    let storage = Arc::clone(&db_storage);
    let client = Arc::clone(&client);
    let exchanges_ids = Arc::clone(&exchanges_ids);
    let exchange_name_owned = exchange_name.to_string();

    set.spawn(async move{
       let interval_string = &interval.to_string();
        let service = TradingService::new(
            exchange_name_owned.clone(),
            exchanges_ids.get(&exchange_name_owned).unwrap().to_owned(),
            interval_string.clone(),
            client,
            storage
        );
        service.run().await;
    });

}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let _ = start_listening().await;
}
