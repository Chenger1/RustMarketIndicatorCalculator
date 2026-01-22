use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.alter_table(
            Table::alter()
                .table("indicators")
                .add_column(
                    ColumnDef::new("indicator_type").string_len(5).not_null()
                )
                .modify_column(
                    ColumnDef::new("value").float().not_null()
                )
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.alter_table(
            Table::alter()
                .table("indicators")
                .drop_column("indicator_type")
                .modify_column(
                    ColumnDef::new("value").integer().not_null()
                )
                .to_owned()
        ).await?;

        Ok(())
    }
}
