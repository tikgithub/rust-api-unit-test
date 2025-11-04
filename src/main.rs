use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web};
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
mod modules;
use modules::{config::Config, db::DBClient};
mod dtos;
mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv().ok();
    env_logger::init();

    let config = Config::init();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let db_client = DBClient::new(pool);

    let app_state = AppState {
        env: config.clone(),
        db_client: db_client.clone(),
    };

    println!("{}", format!("Server is running on port {}", config.port));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(health_checker_handler)
            .wrap(middleware::Logger::default())
    })
    .bind(format!("0.0.0.0:{}", config.port))?
    .run()
    .await?;

    println!("Success Run Application");
    Ok(())
}

#[get("/health-check")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Health Check Work";
    HttpResponse::Ok().json(json!({"status":"success", "message":MESSAGE}))
}
