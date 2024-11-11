use crate::model::clvr_model::CLVRModel;
use crate::trade::implementation::Trade;
use crate::trade::{ITrade, TradeDirection};
use alloy::primitives::U256;

#[cfg(test)]
mod tests {
    use crate::model::Omega;

    use super::*;

    const wei: &str = "000000000000000000";

    fn size(x: u128) -> U256 {
        let x = x.to_string();
        let size: String = x.to_string() + wei;
        U256::from_str_radix(&size, 10).unwrap()
    }

    #[test]
    fn test_clvr() {
        let mut mock_trades: Omega = Omega::new_from(vec![
            Box::new(Trade::new(size(5), TradeDirection::Sell)),
            Box::new(Trade::new(size(10), TradeDirection::Buy)),
            Box::new(Trade::new(size(2), TradeDirection::Sell)),
        ]);

        let expected: Omega = Omega::new_from(vec![
            Box::new(Trade::new(size(2), TradeDirection::Buy)),
            Box::new(Trade::new(size(5), TradeDirection::Sell)),
            Box::new(Trade::new(size(10), TradeDirection::Sell)),
        ]);
        
        let model = CLVRModel::new(size(100), size(100));

        let p_0 = U256::from(1);
        
        model.clvr_order(p_0, &mut mock_trades);
        
        print!("{:?}", mock_trades);
        print!("{:?}", expected);
        
        assert!(mock_trades == expected);
    }
}