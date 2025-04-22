use core::errors::AppError;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Debug)]
pub struct Prefixes {
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RoamInDataDBRecord {
    pub batch_id: i32,
    pub batch_date: String,
    pub hlraddr: String,
    pub nsub: i32,
    pub nsuba: i32,
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RoamOutDataDBRecord {
    pub batch_id: i32,
    pub batch_date: String,
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

pub async fn insert_batch_exec(
    db_client: &Client,
    batch_name: &str,
    source_type: &str,
    source_name: &str,
) -> Result<i32, AppError> {
    let batch_status = "Started";

    let row = db_client
        .query_one(
            "INSERT INTO batch_execs (batch_name, source_type, source_name, start_time, batch_status)
             VALUES ($1, $2, $3, NOW(), $4)
             RETURNING id",
            &[&batch_name, &source_type, &source_name, &batch_status],
        )
        .await?;

    let id: i32 = row.get("id");
    Ok(id)
}

pub async fn update_batch_status(
    db_client: &Client,
    batch_id: i32,
    batch_status: &str,
) -> Result<(), AppError> {
    db_client
        .execute(
            "UPDATE batch_execs
             SET batch_status = $1, end_time = NOW()
             WHERE id = $2",
            &[&batch_status, &batch_id],
        )
        .await?;

    Ok(())
}

pub async fn insert_roam_in_stg_records(
    db_client: &Client,
    records: Vec<RoamInDataDBRecord>,
) -> Result<(), AppError> {
    let query = "
        INSERT INTO stg_roam_in (batch_id, batch_date, hlraddr, nsub, nsuba, prefix , country_id , operator_id )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ";

    for record in records {
        db_client
            .execute(
                query,
                &[
                    &(record.batch_id as i32),
                    &record.batch_date,
                    &record.hlraddr,
                    &(record.nsub as i32),
                    &(record.nsuba as i32),
                    &record.prefix,
                    &record.country_id,
                    &record.operator_id,
                ],
            )
            .await?;
    }

    Ok(())
}

pub async fn insert_roam_out_stg_records(
    db_client: &Client,
    records: Vec<RoamOutDataDBRecord>,
) -> Result<(), AppError> {
    let query = "
        INSERT INTO stg_roam_out (batch_id, batch_date, imsi, msisdn, vlr_number, prefix , country_id , operator_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ";

    for record in records {
        db_client
            .execute(
                query,
                &[
                    &(record.batch_id as i32),
                    &record.batch_date,
                    &record.imsi,
                    &record.msisdn,
                    &record.vlr_number,
                    &record.prefix,
                    &record.country_id,
                    &record.operator_id,
                ],
            )
            .await?;
    }

    Ok(())
}

pub async fn select_all_prefixes(db_client: &Client) -> Result<Vec<Prefixes>, AppError> {
    let query = "
        SELECT prefix, country_id, operator_id FROM prefixes WHERE prefix IS NOT NULL
    ";

    let rows = db_client.query(query, &[]).await.map_err(AppError::from)?;

    let prefixes = rows
        .into_iter()
        .map(|row| Prefixes {
            prefix: row.get(0),
            country_id: row.get(1),
            operator_id: row.get(2),
        })
        .collect();

    Ok(prefixes)
}
