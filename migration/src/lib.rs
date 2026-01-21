pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260120_194751_add_value_indicator_type;
mod m20260120_203238_symbol_id_unique_key;
mod m20260121_200731_remove_symbol_id_unique;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260120_194751_add_value_indicator_type::Migration),
            Box::new(m20260120_203238_symbol_id_unique_key::Migration),
            Box::new(m20260121_200731_remove_symbol_id_unique::Migration),
        ]
    }
}
