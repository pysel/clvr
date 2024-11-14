use std::{fs, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};
use log4rs;

mod clvr;
mod trades;
pub mod server;

async fn cleanup() {
    let log_dir = "log"; // Specify your log directory here
    if let Err(e) = fs::remove_dir_all(log_dir) {
        eprintln!("Error removing log directory: {}", e);
    } else {
        println!("Log directory removed successfully.");
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // load environment variables
    dotenv::dotenv().ok();

    // init logging
    cleanup().await; // cleanup existing log files before starting
    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        eprintln!("Error initializing logging: {}", e);
    }

    let scheduled_db: server::handlers::ScheduledDatabase = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(scheduled_db.clone());
        App::new()
            .app_data(app_data)
            .service(server::handlers::num_trades)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
