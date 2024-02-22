use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Character::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Character::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Food::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Food::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Food::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Machine::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Machine::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Machine::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Mission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Mission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Business::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Business::AccountId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Business::CurrentStageNumber)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(1))),
                    )
                    .col(
                        ColumnDef::new(Business::CurrentStageMoney)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .col(
                        ColumnDef::new(Business::CurrentDia)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .col(
                        ColumnDef::new(Business::EnabledTables)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(1))),
                    )
                    .col(
                        ColumnDef::new(Business::ChefSpeedMultiplier)
                            .float()
                            .not_null()
                            .default(Value::Float(Some(1.0))),
                    )
                    .col(
                        ColumnDef::new(Business::ServerSpeedMultiplier)
                            .float()
                            .not_null()
                            .default(Value::Float(Some(1.0))),
                    )
                    .col(
                        ColumnDef::new(Business::AccumulatedCustomer)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .col(
                        ColumnDef::new(Business::AccumulatedSales)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("account_id")
                            .from(Business::Table, Business::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE TABLE accounts_characters (
                account_id int NOT NULL,
                character_id int NOT NULL,
                current_level int NOT NULL DEFAULT 0,
                PRIMARY KEY (account_id, character_id),
                CONSTRAINT account_id FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT character_id FOREIGN KEY (character_id) REFERENCES character(id) ON DELETE CASCADE ON UPDATE CASCADE
            )"
        )
        .await?;
        db.execute_unprepared(
            "CREATE TABLE accounts_foods (
                account_id int NOT NULL,
                food_id int NOT NULL,
                current_level int NOT NULL DEFAULT 0,
                PRIMARY KEY (account_id, food_id),
                CONSTRAINT account_id FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT food_id FOREIGN KEY (food_id) REFERENCES food(id) ON DELETE CASCADE ON UPDATE CASCADE
            )"
        )
        .await?;
        db.execute_unprepared(
            "CREATE TABLE accounts_machines (
                account_id int NOT NULL,
                machine_id int NOT NULL,
                current_level int NOT NULL DEFAULT 0,
                PRIMARY KEY (account_id, machine_id),
                CONSTRAINT account_id FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT machine_id FOREIGN KEY (machine_id) REFERENCES machine(id) ON DELETE CASCADE ON UPDATE CASCADE
            )"
        )
        .await?;
        db.execute_unprepared(
            "CREATE TABLE accounts_missions (
                account_id int NOT NULL,
                mission_id int NOT NULL,
                is_cleared boolean NOT NULL DEFAULT FALSE,
                PRIMARY KEY (account_id, mission_id),
                CONSTRAINT account_id FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT mission_id FOREIGN KEY (mission_id) REFERENCES mission(id) ON DELETE CASCADE ON UPDATE CASCADE
            )"
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountsMissions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AccountsMachines::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AccountsFoods::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AccountsCharacters::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Business::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Character::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Food::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Machine::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Mission::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Character {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum Food {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum Machine {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum Mission {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Business {
    Table,
    AccountId,
    CurrentStageNumber,
    CurrentStageMoney,
    CurrentDia,
    EnabledTables,
    ChefSpeedMultiplier,
    ServerSpeedMultiplier,
    AccumulatedCustomer,
    AccumulatedSales,
}
#[derive(DeriveIden)]
enum AccountsCharacters {
    Table,
    // UserId,
    // CharacterId,
    // CurrentLevel,
}
#[derive(DeriveIden)]
enum AccountsFoods {
    Table,
    // UserId,
    // FoodId,
    // CurrentLevel,
}
#[derive(DeriveIden)]
enum AccountsMachines {
    Table,
    // UserId,
    // MachineId,
    // CurrentLevel,
}
#[derive(DeriveIden)]
enum AccountsMissions {
    Table,
    // UserId,
    // MissionId,
    // IsCleared,
}
