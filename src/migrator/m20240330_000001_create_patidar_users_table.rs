use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;
use sea_query::extension::postgres::Extension;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240330_000001_create_patidar_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let uuid_extension_stmt = Extension::create()
            .name("\"uuid-ossp\"")
            .schema("public")
            .if_not_exists()
            .to_string(PostgresQueryBuilder);
        let stmt = Statement::from_string(DbBackend::Postgres, uuid_extension_stmt);
        db.execute(stmt).await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Users::FirstName).string())
                    .col(ColumnDef::new(Users::LastName).string())
                    .col(
                        ColumnDef::new(Users::Email)
                            .unique_key()
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}
