use alloy::primitives::U256;


#[derive(PartialEq)]
pub enum TradeDirection {
    Buy,
    Sell
}

pub trait Trade {
    fn get_direction(&self) -> TradeDirection;
    fn get_amount_in(&self) -> U256; // INVARIANT: when direction == Buy, amount_in is in tokens y, when direction == Sell, amount_in is in tokens x
}