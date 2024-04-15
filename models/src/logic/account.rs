//
//  Copyright 2024 Ram Flux, LLC.
//

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: i64,
    pub account: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Account {
    pub async fn create(
        pool: &sqlx::PgPool,
        new_account: &Account,
    ) -> Result<Account, crate::Error> {
        let get_current_date = Utc::now().naive_utc();

        let mut tx = pool.begin().await?;
        let account = sqlx::query_as!(
            Account,
            r#"
            INSERT INTO account ( account,created_at,updated_at)
            VALUES ($1, $2, $3)
            RETURNING *;
            "#,
            new_account.account,
            Some(get_current_date),
            Some(get_current_date),
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(account)
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, account_id: i64) -> Result<Account, crate::Error> {
        let account = sqlx::query_as!(
            Account,
            r#"
            SELECT * FROM account WHERE id = $1;
            "#,
            account_id
        )
        .fetch_one(pool)
        .await?;
        Ok(account)
    }

    pub async fn get_by_pubkey(
        pool: &sqlx::PgPool,
        account: &str,
    ) -> Result<Option<Account>, crate::Error> {
        let result = sqlx::query_as!(
            Account,
            r#"
            SELECT * FROM account WHERE account = $1;
            "#,
            account
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(result) => Ok(result),
            Err(e) => {
                println!("error:{:#?}", e);
                Ok(None)
            }
        }
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        by_id: i64,
        account: &str,
    ) -> Result<(), crate::Error> {
        sqlx::query!(
            r#"
            UPDATE account SET account = $1 WHERE id = $2;
            "#,
            account,
            by_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &sqlx::PgPool, account_id: i64) -> Result<(), crate::Error> {
        sqlx::query!(
            r#"
            DELETE FROM account WHERE id = $1;
            "#,
            account_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Account>, crate::Error> {
        let accounts = sqlx::query_as!(
            Account,
            r#"
            SELECT * FROM account;
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(accounts)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use dotenv::dotenv;
//     use sqlx::PgPool;
//     use std::env;

//     async fn setup_database() -> PgPool {
//         dotenv().ok(); 
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         PgPool::connect(&database_url)
//             .await
//             .expect("Failed to connect to database")
//     }

//     #[tokio::test]
//     async fn test_create_account() {
//         let pool = setup_database().await;
//         let unique_account = format!("test_account_{}", Utc::now().timestamp());
//         let account = Account::create(
//             &pool,
//             &Account {
//                 id: 0,
//                 account: unique_account.clone(),
//                 created_at: None,
//                 updated_at: None,
//             },
//         )
//         .await
//         .expect("Failed to create account");

//         // assert_eq!(account.account, unique_account);
//         let updated_name = "Updated Test Name";
//         Account::update(&pool, account.id, updated_name)
//             .await
//             .expect("Failed to update account");

//         let updated_account = Account::get_by_id(&pool, account.id)
//             .await
//             .expect("Failed to fetch updated account");

//         assert_eq!(updated_account.account, updated_name);

//     }

//     #[tokio::test]
//     async fn test_get_account() {
//         let pool = setup_database().await;

//         let account_id = 1;
//         let account = Account::get_by_id(&pool, account_id)
//             .await
//             .expect("Failed to get account");

//         assert_eq!(account.id, account_id);
//     }

//     #[tokio::test]
//     async fn test_get_pubk_account() {
//         let pool = setup_database().await;

//         let account = "test_public_key";
//         let account = Account::get_by_pubkey(&pool, account)
//             .await
//             .expect("Failed to get account");
//         println!(":{:#?}", account);
//     }

//     #[tokio::test]
//     async fn test_get_all_accounts() {
//         let pool = setup_database().await;
//         let accounts = Account::get_all(&pool)
//             .await
//             .expect("Failed to get accounts");
//         println!("{:?}", accounts);
//         // assert!(!accounts.is_empty());
//     }
// }
