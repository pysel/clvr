use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use serde::{Serialize, Deserialize};
use server::handlers::ScheduleRequest;

mod clvr;
mod trades;
mod server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    let server = server::Processor::new();
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
