use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct FileStatus {
    last_load: String,
    filename: String,
    status: String,
}

#[derive(Serialize)]
struct FileStatusResponse {
    data: Vec<FileStatus>,
}

#[derive(Serialize)]
struct OverviewStats {
    roamin: i64,
    roamout: i64,
    alerts: i64,
    notifications: i64,
}

#[derive(Serialize)]
struct OverviewResponse {
    data: Vec<OverviewStats>,
}

#[derive(Serialize)]
struct TopCountry {
    country: String,
    Nombre: i64,
    Total: i64,
}

#[derive(Serialize)]
struct TopCountryResponse {
    data: Vec<TopCountry>,
}

#[get("/api/v1/analytics/status")]
async fn get_status() -> impl Responder {
    let data = vec![
        FileStatus {
            last_load: "2025-04-19T10:30:00Z".to_string(),
            filename: "roam_in_20250418.csv".to_string(),
            status: "Success".to_string(),
        },
        FileStatus {
            last_load: "2025-04-19T10:45:00Z".to_string(),
            filename: "roam_out_20250418.csv".to_string(),
            status: "Failed".to_string(),
        },
    ];

    HttpResponse::Ok().json(FileStatusResponse { data })
}

#[get("/api/v1/analytics/overview")]
async fn get_overview() -> impl Responder {
    let stats = OverviewStats {
        roamin: 27530,
        roamout: 58640,
        alerts: 12,
        notifications: 4,
    };

    HttpResponse::Ok().json(OverviewResponse {
        data: vec![stats],
    })
}

#[get("/api/v1/analytics/top-roamin")]
async fn get_top_roamin() -> impl Responder {
    let data = vec![
        TopCountry { country: "France".into(), Nombre: 23000, Total: 54320 },
        TopCountry { country: "Libya".into(), Nombre: 12406, Total: 54320 },
        TopCountry { country: "Algeria".into(), Nombre: 6530, Total: 54320 },
        TopCountry { country: "Qatar".into(), Nombre: 2011, Total: 54320 },
        TopCountry { country: "Belgium".into(), Nombre: 516, Total: 54320 },
    ];

    HttpResponse::Ok().json(TopCountryResponse { data })
}

#[get("/api/v1/analytics/top-roamout")]
async fn get_top_roamout() -> impl Responder {
    let data = vec![
        TopCountry { country: "France".into(), Nombre: 23000, Total: 54320 },
        TopCountry { country: "Libya".into(), Nombre: 12406, Total: 54320 },
        TopCountry { country: "Algeria".into(), Nombre: 6530, Total: 54320 },
        TopCountry { country: "Qatar".into(), Nombre: 2011, Total: 54320 },
        TopCountry { country: "Belgium".into(), Nombre: 516, Total: 54320 },
    ];

    HttpResponse::Ok().json(TopCountryResponse { data })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .service(get_status)
            .service(get_overview)
            .service(get_top_roamin)
            .service(get_top_roamout)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
