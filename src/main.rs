use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer, web, HttpResponse, http::header::ContentType,};

mod data_store;
mod repositories;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));



    data_store::load_work_items_all().await;


    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // .service(Files::new("/images", "static/images/").show_files_listing())
            .route("api/categories", web::get().to(get_categories))
            .route("api/fields", web::get().to(get_fields))
            .route("api/classification", web::get().to(get_classification))
            .route("api/states", web::get().to(get_states))
            .service(Files::new("/", "./dist/web-frontend/browser").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}