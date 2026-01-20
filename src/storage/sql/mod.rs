use std::collections::HashMap;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveValue, ActiveModelTrait, Statement, DatabaseBackend, ConnectionTrait};
use crate::storage::{Storage};
use crate::entity::{exchanges, symbols, indicators};
use crate::structs::Indicator;

#[derive(Clone)]
pub struct DBStorage{
    pool: DatabaseConnection,
}

impl DBStorage{
    pub fn new(pool: DatabaseConnection) -> Self{
        DBStorage { pool }
    }
}

impl Storage for DBStorage{
    async fn save_indicators(&self, indicators_data: Vec<Indicator>){
        let symbols_ids: Vec<i32> = indicators_data.iter().map(|indicator| indicator.symbol_id).collect();
        let models: HashMap<i32, indicators::Model> = indicators::Entity::find()
            .filter(indicators::Column::Id.is_in(symbols_ids))
            .all(&self.pool)
            .await
            .unwrap()
            .iter()
            .map(|model| (model.symbol_id, model.clone()))
            .collect();

        let mut to_create: Vec<indicators::ActiveModel> = Vec::new();
        let mut to_update: Vec<String> = Vec::new();
        let mut to_update_ids: Vec<String> = Vec::new();
        for indicator in indicators_data{
            if let Some(_) = models.get(&indicator.symbol_id){
                to_update.push(format!("WHEN {} THEN {}", indicator.symbol_id, indicator.value as i32));
                to_update_ids.push(indicator.symbol_id.to_string());
            }else{
                to_create.push(
                    indicators::ActiveModel{
                        id: ActiveValue::NotSet,
                        symbol_id: ActiveValue::Set(indicator.symbol_id),
                        value: ActiveValue::Set(indicator.value as i32)
                    }
                )
            }
        }

        indicators::Entity::insert_many(to_create)
            .exec(&self.pool)
            .await
            .unwrap();

        let sql = format!(
            "UPDATE indicators SET value = CASE id {} END WHERE id IN ({})",
            to_update.join(" "),
            to_update_ids.join(", ")
        );

        self.pool.execute_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            sql,
        ))
            .await.unwrap();
    }

    async fn create_exchange(&self, title: &String) -> i32 {
        let exchange: Option<exchanges::Model> = exchanges::Entity::find()
            .filter(exchanges::Column::Title.eq(title))
            .one(&self.pool).await.unwrap();
        if let Some(e) = exchange{
            return e.id
        }
        let exchange = exchanges::ActiveModel{
            title: ActiveValue::Set(title.clone()),
            id: ActiveValue::NotSet
        };
        let exchange: exchanges::Model = exchange.insert(&self.pool).await.unwrap();
        exchange.id
    }

    async fn create_symbols(&self, symbols: Vec<String>, exchange_id: i32) -> Vec<symbols::Model>{
        symbols::Entity::delete_many().exec(&self.pool).await.unwrap();
        let to_create = symbols.into_iter()
            .map(|symbol| {
                symbols::ActiveModel{
                    id: ActiveValue::NotSet,
                    exchange_id: ActiveValue::Set(exchange_id),
                    tile: ActiveValue::Set(symbol)
                }
            }).collect::<Vec<symbols::ActiveModel>>();

        symbols::Entity::insert_many(to_create).exec_with_returning(&self.pool).await.unwrap()
    }
}
