use actix_web::HttpResponse;
use alloy::primitives::U256;
use alloy::transports::http::Http;
use crate::server::Server;
use crate::trades::implementation::Trade;

use crate::trades::{ITrade, TradeDirection};

impl Server {
    pub async fn submit_trade(&mut self, amount_in: &str, direction: &str) -> HttpResponse {
        let amount_in = U256::from_str_radix(amount_in, 10).unwrap();
        let direction = match direction {
            "buy" => TradeDirection::Buy,
            "sell" => TradeDirection::Sell,
            _ => return HttpResponse::BadRequest().into(),
        };

        let trade: Box<dyn ITrade> = Box::new(Trade::new(amount_in, direction));
        self.add_trade(trade);

        HttpResponse::Ok().into()
    }

    pub async fn set_reserves(&mut self, reserve_x: &str, reserve_y: &str) -> HttpResponse {
        let reserve_x = U256::from_str_radix(reserve_x, 10).unwrap();
        let reserve_y = U256::from_str_radix(reserve_y, 10).unwrap();

        self.model.set_reserves(reserve_x, reserve_y);

        HttpResponse::Ok().into()
    }

    pub async fn run_clvr(&mut self, p_0: &str) -> HttpResponse {
        let p_0 = U256::from_str_radix(p_0, 10).unwrap();
        
        self.model.clvr_order(p_0, &mut self.omega);

        HttpResponse::Ok().into()
    }
}