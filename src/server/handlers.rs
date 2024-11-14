use std::{str::FromStr, sync::{Arc, Mutex}};

use actix_web::{get, post, web, HttpResponse, Responder};
use alloy::primitives::{Address, FixedBytes, PrimitiveSignature, U256};
use log::{info, warn};
use crate::server::handlers_types::*;

use crate::server::{eip2612::verify_eip2612_signature};

pub type ScheduledDatabase = Arc<Mutex<Vec<ScheduledTrade>>>;

#[get("/num_trades")]
pub async fn num_trades(db: web::Data<ScheduledDatabase>) -> impl Responder {
    info!(target: "server::handlers", "num_trades called");
    HttpResponse::Ok().json(NumTradesResponse {
        num_trades: db.lock().unwrap().len() as u64,
    })
}

#[post("/submit_trade")]
pub async fn submit_trade(trade_request: web::Json<ScheduleRequest>, db: web::Data<ScheduledDatabase>,) -> impl Responder {
    info!(target: "server::handlers", "submit_trade called");

    let mut db = db.lock().unwrap();

    // verify from address
    let from = Address::from_str(&trade_request.from).unwrap_or(Address::ZERO);
    if from == Address::ZERO {
        warn!(target: "server::handlers", "Invalid from address");
        return HttpResponse::BadRequest().json(ScheduleResponse {
            success: false,
            message: "Invalid from address".to_string(),
        });
    }
    
    // verify signature (return default types except panicking so that verification fails gracefully)
    let permit_message: FixedBytes<32> = FixedBytes::from_str(&trade_request.permit_msg)
        .unwrap_or(FixedBytes::ZERO);
    let signature: PrimitiveSignature = PrimitiveSignature::from_str(&trade_request.signature)
        .unwrap_or(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false));
    let signer: Address = Address::from_str(&trade_request.from)
        .unwrap_or(Address::ZERO);
    
    if !verify_eip2612_signature(permit_message, signature, signer) {
        warn!(target: "server::handlers", "Invalid signature");
        return HttpResponse::BadRequest().json(ScheduleResponse {
            success: false,
            message: "Invalid signature".to_string(),
        });
    }
    
    let scheduled_trade: ScheduledTrade = trade_request.into_inner().into();
    db.push(scheduled_trade);

    HttpResponse::Created().json(ScheduleResponse {
        success: true,
        message: "Trade submitted".to_string(),
    })
}
