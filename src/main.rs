use alloy::primitives::U256;
use clvr::model::Omega;
use trades::{implementation::Trade, TradeDirection};

mod clvr;
mod trades;
mod server;

fn main() {
    let mut model = clvr::model::clvr_model::CLVRModel::new();
    model.set_reserves(U256::from(0), U256::from(0));

    let mut omega = Omega::new();
    omega.push(Box::new(Trade::new(U256::from(100), TradeDirection::Buy)));
    omega.push(Box::new(Trade::new(U256::from(100), TradeDirection::Sell)));
    
    model.clvr_order(U256::from(100), &mut omega);
}
