use std::{str::FromStr, sync::{Arc, Mutex}};

use actix_web::{get, post, web, HttpResponse, Responder};
use alloy::primitives::{Address, FixedBytes, PrimitiveSignature, U256};
use serde::{Deserialize, Serialize};

use crate::server::eip2612::verify_eip2612_signature;


pub type ScheduledDatabase = Arc<Mutex<Vec<ScheduleRequest>>>;

#[derive(Serialize, Deserialize)]
pub struct ScheduleRequest {
    from: String,
    /*
    swap_params
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    encoded as a json string
     */
    swap_params: String, 
    permit_msg: String,
    signature: String,
}

#[get("/num_trades")]
pub async fn num_trades() -> impl Responder {
    format!("Hi")
}

#[post("/submit_trade")]
pub async fn submit_trade(trade_request: web::Json<ScheduleRequest>, db: web::Data<ScheduledDatabase>) -> impl Responder {
    let mut db = db.lock().unwrap();
    
    // verify signature (return default types except panicking so that verification fails gracefully)
    let permit_message: FixedBytes<32> = FixedBytes::from_str(&trade_request.permit_msg)
        .unwrap_or(FixedBytes::ZERO);
    let signature: PrimitiveSignature = PrimitiveSignature::from_str(&trade_request.signature)
        .unwrap_or(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false));
    let signer: Address = Address::from_str(&trade_request.from)
        .unwrap_or(Address::ZERO);
    
    if !verify_eip2612_signature(permit_message, signature, signer) {
        return HttpResponse::BadRequest().body("Invalid signature");
    }

    db.push(trade_request.into_inner());
    HttpResponse::Ok().body("Trade submitted")
}
