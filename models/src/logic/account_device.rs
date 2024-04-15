//
//  Copyright 2024 Ram Flux, LLC.
//


use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct AccountDevice {
    pub id: i64,
    pub account: Option<String>,
    pub public_key: Option<String>,
    pub pubkey: Option<String>,
    pub prikey: Option<String>,
    pub def: Option<String>,
    pub token: Option<String>,
    pub proof: Option<String>,
    pub versions: Option<String>,
    pub ext: Option<String>,
    pub last_ip: Option<String>,
    pub osrng: Option<String>,
    //1/not binding 2/binding
    pub status: Option<i16>,
    pub last_time: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl AccountDevice {
    pub async fn create(
        pool: &sqlx::PgPool,
        new_account_device: &AccountDevice,
    ) -> Result<AccountDevice, crate::Error> {
        let get_current_date = Utc::now().naive_utc();
        let mut tx = pool.begin().await?;
        let account_device = sqlx::query_as!(
            AccountDevice,
            r#"
            INSERT INTO account_device (account,public_key, pubkey, prikey, def, token, proof, versions, ext, last_ip, last_time, created_at, updated_at,osrng,status)
            VALUES ($1,$2,$3, $4, $5,  $6, $7, $8, $9, $10, $11, $12, $13,$14,$15)
            RETURNING *;
            "#,
            new_account_device.account,
            new_account_device.public_key,
            new_account_device.pubkey,
            new_account_device.prikey,
            new_account_device.def,
            new_account_device.token,
            new_account_device.proof,
            new_account_device.versions,
            new_account_device.ext,
            new_account_device.last_ip,
            new_account_device.last_time,
            Some(get_current_date),
            Some(get_current_date),
            new_account_device.osrng,
            new_account_device.status,
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(account_device)
    }

    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        account_device_id: i64,
    ) -> Result<AccountDevice, crate::Error> {
        let account_device = sqlx::query_as!(
            AccountDevice,
            r#"
            SELECT * FROM account_device WHERE id = $1;
            "#,
            account_device_id
        )
        .fetch_one(pool)
        .await?;
        Ok(account_device)
    }

    pub async fn init_update(
        pool: &sqlx::PgPool,
        pubkey: &str,
        prikey: &str,
        osrng: &str,
        updated_at: &NaiveDateTime,
        proof: &str,
        account: &str,
    ) -> Result<bool, crate::Error> {
        let result =sqlx::query!(
            r#"
            UPDATE account_device SET pubkey = $1,prikey=$2 ,osrng=$3,updated_at=$4 WHERE proof = $5 AND account = $6;
            "#,
            Some(pubkey),
            Some(prikey),
            Some(osrng),
            Some(updated_at),
            proof,
            account,
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn binding_update(
        pool: &sqlx::PgPool,
        public_key: &str,
        updated_at: &NaiveDateTime,
        proof: &str,
        account: &str,
    ) -> Result<bool, crate::Error> {
        let result =sqlx::query!(
            r#"
            UPDATE account_device SET public_key=$1,osrng=$2,updated_at=$3,status=$4 WHERE proof = $5 AND account = $6;
            "#,
            Some(public_key),
            Some("".to_string()),
            Some(updated_at),
            Some(2),
            proof,
            account,
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_by_pubkey(
        pool: &sqlx::PgPool,
        public_key: &str,
    ) -> Result<AccountDevice, crate::Error> {
        let account_device = sqlx::query_as!(
            AccountDevice,
            r#"
            SELECT * FROM account_device WHERE public_key = $1;
            "#,
            public_key
        )
        .fetch_one(pool)
        .await?;
        Ok(account_device)
    }

    /**
     * the by server in pubkey
     */
    pub async fn get_by_pubkey_osrng(
        pool: &sqlx::PgPool,
        pubkey: &str,
        osrng: &str,
    ) -> Result<Option<AccountDevice>, crate::Error> {
        let account_device = sqlx::query_as!(
            AccountDevice,
            r#"
            SELECT * FROM account_device WHERE pubkey=$1 AND osrng=$2;
            "#,
            pubkey,
            osrng,
        )
        .fetch_optional(pool)
        .await?;
        Ok(account_device)
    }

    pub async fn get_by_proof(
        pool: &sqlx::PgPool,
        public_key: &str,
    ) -> Result<Option<AccountDevice>, crate::Error> {
        let result = sqlx::query_as!(
            AccountDevice,
            r#"
            SELECT * FROM account_device WHERE proof = $1;
            "#,
            public_key
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

    pub async fn delete_by_pubkey(
        pool: &sqlx::PgPool,
        public_key: &str,
    ) -> Result<(), crate::Error> {
        sqlx::query!(
            r#"
            DELETE FROM account_device WHERE public_key = $1;
            "#,
            public_key
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
