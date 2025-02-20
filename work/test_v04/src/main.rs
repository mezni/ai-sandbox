use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tracing_subscriber;
use prometheus::{Encoder, TextEncoder, Registry, IntCounter, Gauge};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt::init();
    info!("Starting Server");

    // Set up Prometheus metrics
    let registry = Registry::new();
    let request_counter = IntCounter::new("http_requests_total", "Total number of HTTP requests").unwrap();
    let response_latency = Gauge::new("http_response_latency_seconds", "Response latency in seconds").unwrap();

    // Register metrics
    registry.register(Box::new(request_counter.clone())).unwrap();
    registry.register(Box::new(response_latency.clone())).unwrap();

    // Flag to track if event generation has already been called
    let event_generated = Arc::new(Mutex::new(false));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(request_counter.clone()))
            .app_data(web::Data::new(response_latency.clone()))
            .app_data(web::Data::new(event_generated.clone())) // Share the event_generated flag
            .service(health_check)
            .service(generate_event)
            .service(metrics) // Add the metrics endpoint
    })
    .bind(("0.0.0.0", 3000))?;

    let server_handle = tokio::spawn(server.run());

    // Wait for a termination signal (e.g., Ctrl+C)
    signal::ctrl_c().await.expect("Failed to install Ctrl+C signal handler");
    info!("Received Ctrl+C signal, shutting down...");

    // Await the server handle to ensure graceful shutdown
    server_handle.await.unwrap();

    Ok(())
}

// Health check endpoint
#[get("/health")]
async fn health_check() -> impl Responder {
    info!("Health check endpoint was hit");
    HttpResponse::Ok().json("Service is healthy")
}

// Generate event endpoint
#[get("/generate")]
async fn generate_event(event_generated: web::Data<Arc<Mutex<bool>>>) -> impl Responder {
    let mut generated = event_generated.lock().unwrap(); // Lock the mutex

    if *generated {
        info!("Generate event endpoint was already hit; ignoring subsequent requests");
        return HttpResponse::Ok().json("Event generation already executed");
    }

    // Mark the event as generated
    *generated = true;

    info!("Generate event endpoint was hit for the first time");
    // You can add the logic for event generation here
    HttpResponse::Ok().json("Event generation executed successfully")
}

// Metrics endpoint
#[get("/metrics")]
async fn metrics(
    request_counter: web::Data<IntCounter>,
    response_latency: web::Data<Gauge>,
) -> impl Responder {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    // Increment the request counter
    request_counter.inc();

    // Here you can simulate or record the response latency
    response_latency.set(0.1); // Set latency as a floating-point value

    // Encode metrics to a string
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok().body(buffer)
}
