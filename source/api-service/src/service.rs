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
pub struct NotificationsResponse {
    pub notification: String,
}

#[derive(Serialize)]
pub struct AnomaliesResponse {
    pub name_en: String,
    pub operator: String,
    pub country_count: String,
    pub operator_count: String,
    pub configure: String,
    pub reel: String,
    pub routage: String,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub date: String,
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

pub async fn count_roam_out_operators_service(
    db: &DBManager,
) -> Result<Vec<StatsResponse>, AppError> {
    let client = db.get_client().await?;
    let raw = repo::count_roam_out_operators(&client).await?;

    let wrapped = raw
        .into_iter()
        .map(|(date, country, operator, count)| StatsResponse {
            date,
            country: Some(country),
            operator: Some(operator),
            count,
        })
        .collect();

    Ok(wrapped)
}

pub async fn count_roam_out_countries_service(
    db: &DBManager,
) -> Result<Vec<StatsResponse>, AppError> {
    let client = db.get_client().await?;
    let raw = repo::count_roam_out_countries(&client).await?;

    let wrapped = raw
        .into_iter()
        .map(|(date, country, count)| StatsResponse {
            date,
            country: Some(country),
            operator: None,
            count,
        })
        .collect();

    Ok(wrapped)
}

pub async fn count_roam_out_dates_service(db: &DBManager) -> Result<Vec<StatsResponse>, AppError> {
    let client = db.get_client().await?;
    let raw = repo::count_roam_out_dates(&client).await?;

    let wrapped = raw
        .into_iter()
        .map(|(date, count)| StatsResponse {
            date,
            country: None,
            operator: None,
            count,
        })
        .collect();

    Ok(wrapped)
}

pub async fn get_notifications_service(
    db: &DBManager,
) -> Result<Vec<NotificationsResponse>, AppError> {
    let client = db.get_client().await?;
    let raw = repo::get_notifications(&client).await?;

    let wrapped = raw
        .into_iter()
        .map(|notification| NotificationsResponse { notification })
        .collect();

    Ok(wrapped)
}


pub async fn get_anomalie_sor_service(db: &DBManager) -> Result<Vec<AnomaliesResponse>, AppError> {
    let client = db.get_client().await?;
    let anomalies = repo::get_anomalie_sor(&client).await?;

    let responses: Vec<AnomaliesResponse> = anomalies
        .into_iter()
        .map(|(name_en, operator, country_count, operator_count, configure, reel, routage)| {
            AnomaliesResponse {
                name_en,
                operator,
                country_count,
                operator_count,
                configure,
                reel,
                routage,
            }
        })
        .collect();

    Ok(responses)
}
