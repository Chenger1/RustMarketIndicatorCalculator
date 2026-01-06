pub mod bybit;
pub mod api_client;

pub fn get_exchange_client(exchange: &api_client::ExchangeEnum) -> Box<dyn api_client::ApiClient>{
    match exchange {
        api_client::ExchangeEnum::BybitFutures => {
            Box::new(bybit::BybitApiClient::new(String::from("linear")))
        }
        api_client::ExchangeEnum::BybitSpot => {
            Box::new(bybit::BybitApiClient::new(String::from("spot")))
        }
    }
}
