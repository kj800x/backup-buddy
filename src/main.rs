use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod db;
mod worker;
use db::dao::PolicyDao;
use db::init_db;
use worker::Worker;
mod config;
mod pages;

macro_rules! serve_static_file {
    ($file:expr) => {
        web::resource(concat!("backup-buddy/assets/", $file)).route(web::get().to(|| async move {
            let path = Path::new("src/res").join($file);

            if path.exists() && path.is_file() {
                let mut file = File::open(path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                HttpResponse::Ok()
                    .append_header(("x-resource-source", "disk"))
                    .body(contents)
            } else {
                HttpResponse::Ok()
                    .append_header(("x-resource-source", "embedded"))
                    .body(include_str!(concat!("res/", $file)))
            }
        }))
    };
}

pub struct AppState {
    pub policy_dao: Arc<PolicyDao>,
    pub config: Arc<config::Config>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_level(true)
        .with_ansi(true)
        // .pretty()
        .init();

    info!("Starting Backup Buddy server");

    // Initialize configuration
    let config = config::Config::from_env().expect("Failed to load configuration");
    info!(db_path = %config.db_path.display(), "Configuration loaded");

    let conn = init_db(&config.db_path).expect("Failed to initialize database");

    info!("Database initialized successfully");

    let policy_dao = Arc::new(PolicyDao::new(conn));
    let app_state = web::Data::new(AppState {
        policy_dao: policy_dao.clone(),
        config: Arc::new(config.clone()),
    });

    // Spawn the controller thread
    let policy_dao_clone = policy_dao.clone();
    let config_clone = Arc::new(config.clone());
    let handle = tokio::spawn(async move {
        let worker = Worker::new(policy_dao_clone, config_clone);
        worker.run().await;
        panic!("Worker thread returned unexpectedly");
    });

    info!("Starting HTTP server on 0.0.0.0:8080");

    // Run both the HTTP server and controller
    let server = HttpServer::new(move || {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(app_state.clone())
            .service(serve_static_file!("styles.css"))
            .service(serve_static_file!("htmx.min.js"))
            .service(serve_static_file!("idiomorph.min.js"))
            .service(serve_static_file!("idiomorph-ext.min.js"))
            .route(
                "/",
                web::get().to(|| async move {
                    HttpResponse::TemporaryRedirect()
                        .append_header(("Location", "/backup-buddy"))
                        .body("")
                }),
            )
            .service(
                web::scope("/backup-buddy")
                    .route("", web::get().to(pages::handlers::web_index))
                    .route(
                        "/index-fragment",
                        web::get().to(pages::handlers::web_index_fragment),
                    )
                    .route(
                        "/policy/new",
                        web::get().to(pages::handlers::create_policy_form),
                    )
                    .route(
                        "/policy/create",
                        web::post().to(pages::handlers::create_policy),
                    ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    tokio::select! {
        _ = server => panic!("HTTP server panicked or returned unexpectedly"),
        _ = handle => panic!("Worker thread panicked or returned unexpectedly")
    }
}
