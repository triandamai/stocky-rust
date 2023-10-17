use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInfo::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserInfo::UserId).string().not_null())
                    .col(ColumnDef::new(UserInfo::Name).string().not_null())
                    .col(ColumnDef::new(UserInfo::Value).string().not_null())
                    .col(ColumnDef::new(UserInfo::CreatedAt).date_time())
                    .col(ColumnDef::new(UserInfo::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
   

       todo!()
    }
}

#[derive(DeriveIden)]
enum UserInfo {
    Table,
    Id,
    UserId,
    Name,
    Value,
    CreatedAt,
    UpdatedAt
}
