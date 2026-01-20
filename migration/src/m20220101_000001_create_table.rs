use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("exchanges")
                    .if_not_exists()
                    .col(ColumnDef::new("id").integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new("title").string().not_null())
                    .to_owned()
            ).await?;

        let symbols = Table::create()
            .table("symbols")
            .if_not_exists()
            .col(ColumnDef::new("id").integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new("tile").string().not_null())
            .col(ColumnDef::new("exchange_id").integer().not_null())
            .foreign_key(ForeignKey::create()
                .name("exchange_fk")
                .from("symbol", "exchange_id")
                .to("exchanges", "id")
                .on_delete(ForeignKeyAction::Cascade))
            .to_owned();

        manager.create_table(symbols).await?;
        manager.create_index(
            Index::create()
                .if_not_exists()
                .name("exchange_fk_index")
                .table("symbols")
                .col("exchange_id")
                .to_owned()
        ).await?;

        let indicators = Table::create()
            .table("indicators")
            .if_not_exists()
            .col(ColumnDef::new("id").integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new("symbol_id").integer().not_null())
            .foreign_key(ForeignKey::create()
                .name("symbol_fk")
                .from("indicators", "symbol_id")
                .to("symbols", "id")
                .on_delete(ForeignKeyAction::Cascade))
            .to_owned();

        manager.create_table(indicators).await?;
        manager.create_index(
            Index::create()
                .if_not_exists()
                .name("symbol_fk_index")
                .table("indicators")
                .col("symbol_id")
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table("exchanges").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("symbols").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("indicators").to_owned())
            .await?;

        Ok(())
    }
}
