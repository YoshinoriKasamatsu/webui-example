use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer, web, HttpResponse, http::header::ContentType,};
use azure_devops_rust_lib::models::config::Config;
use serde::{Deserialize, Serialize};
use tokio::fs;
use chrono::{DateTime, Duration, Utc};

extern crate azure_devops_rust_lib;


mod data_store;
mod repositories;
mod data_operation;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub organization: String,
    pub project: String,
    pub repository_id: String,
    pub pat: String,
    pub output_path: String,
    pub start_date: String,
    pub duration_days: i64,

}

async fn get_work_items() -> HttpResponse {

    let response = repositories::get_work_items().await;
    let json_text = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_text)
}

async fn get_categories() -> HttpResponse {

    let categories = repositories::get_categories().await;
    let json_text = serde_json::to_string(&categories).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_text)
}

async fn get_fields() -> HttpResponse {

    let values = repositories::get_fields().await;
    let json_text = serde_json::to_string(&values).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_text)
}

async fn get_work_item_types() -> HttpResponse {

    let values = repositories::get_work_item_types().await;
    let json_text = serde_json::to_string(&values).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_text)
}

async fn get_classification() -> HttpResponse {

    let values = repositories::get_classification().await;
    let json_text = serde_json::to_string(&values).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_text)
}

async fn get_states() -> HttpResponse {

    let values = repositories::get_states().await;
    let json_text = serde_json::to_string(&values).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_text)
}

async fn post_operation_load_data() -> HttpResponse {
    let app_config: AppConfig = get_app_config().await;
    let config: Config = get_config(&app_config);

    data_operation::load_data(&app_config.output_path, &config).await;
    HttpResponse::Ok()
        .finish()
}

async fn get_app_config() -> AppConfig {
    let contents = fs::read_to_string("config.toml").await.unwrap();
    let app_config: AppConfig = toml::from_str(&contents).unwrap();
    app_config
}

fn get_config(app_config: &AppConfig) -> Config {
    return Config {
        organization: app_config.organization.clone(),
        project: app_config.project.clone(),
        repository_id: app_config.repository_id.clone(),
        pat: app_config.pat.clone(),
        start_date: app_config.start_date.clone(),
        duration_days: app_config.duration_days
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    let app_config: AppConfig = get_app_config().await;
    println!("{:?}", &app_config);


    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // .service(Files::new("/images", "static/images/").show_files_listing())
            .route("api/workitems", web::get().to(get_work_items))
            .route("api/categories", web::get().to(get_categories))
            .route("api/fields", web::get().to(get_fields))
            .route("api/workitemtypes", web::get().to(get_work_item_types))
            .route("api/classification", web::get().to(get_classification))
            .route("api/states", web::get().to(get_states))
            .route("api/data-operation/load-data", web::post().to(post_operation_load_data))
            .service(Files::new("/", "./dist/web-frontend/browser").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}