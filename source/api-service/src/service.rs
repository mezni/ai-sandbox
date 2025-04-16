use crate::repo;
use core::db::DBManager;
use core::errors::AppError;
use serde::Serialize;
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
