use crate::repo;
use core::db::DBManager;
use core::errors::AppError;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct OverviewResponse {
    pub last_date: String,
    pub count_roam_in: i64,
    pub count_roam_out: i64,
    pub count_anomalies: i64,
    pub count_notifications: i64,
}
#[derive(Serialize)]
pub struct StatsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    pub count: i64,
}

pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Health check passed".to_string(),
    }
}

pub async fn overview_service(db: &DBManager) -> Result<OverviewResponse, AppError> {
    let client = db.get_client().await?;
    let last_date = repo::last_date(&client).await?;
    let count_roam_out = repo::count_last_roam_out(&client).await?;
    let count_roam_in = repo::count_last_roam_in(&client).await?;
    let count_anomalies = repo::count_anomalies(&client).await?;
    let count_notifications = repo::count_notifications(&client).await?;

    Ok(OverviewResponse {
        last_date,
        count_roam_in,
        count_roam_out,
        count_anomalies,
        count_notifications,
    })
}

pub async fn stats_service(
    db: &DBManager,
    fact_table: &str,
    dimensions: &[String],
) -> Result<Vec<StatsResponse>, AppError> {
    let select_clause = if dimensions.is_empty() {
        "COUNT(*) as count".to_string()
    } else {
        format!("{}, COUNT(*) as count", dimensions.join(", "))
    };

    let group_by_clause = if dimensions.is_empty() {
        "".to_string()
    } else {
        format!("GROUP BY {}", dimensions.join(", "))
    };

    let query = format!(
        "SELECT {} FROM {} {} ORDER BY count DESC",
        select_clause, fact_table, group_by_clause
    );

    let client = db.get_client().await?;
    let raw = repo::fetch_stats(&client, query).await?;

    let wrapped = raw
        .into_iter()
        .map(|(date, country, operator, count)| StatsResponse {
            date,
            country,
            operator,
            count,
        })
        .collect();

    Ok(wrapped)
}
