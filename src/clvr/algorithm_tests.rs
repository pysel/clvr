use crate::model::clvr_model::CLVRModel;
use crate::trade::implementation::Trade;
use crate::trade::{ITrade, TradeDirection};
use alloy::primitives::U256;

#[cfg(test)]
mod tests {
    use std::ptr::null;

    use alloy::hex::NIL;

    use crate::model::Omega;

    use super::*;

    const WEI: &str = "000000000000000000";

    fn size(x: u128) -> U256 {
        let x = x.to_string();
        let size: String = x.to_string() + WEI;
        U256::from_str_radix(&size, 10).unwrap()
    }

    #[test]
    fn test_clvr() {
        let mut mock_omega: Omega = Omega::new_from(vec![
            Box::new(Trade::new(size(5), TradeDirection::Sell)),
            Box::new(Trade::new(size(10), TradeDirection::Buy)),
            Box::new(Trade::new(size(2), TradeDirection::Sell)),
        ]);

        let expected: Omega = Omega::new_from(vec![
            Box::new(Trade::new(size(2), TradeDirection::Sell)),
            Box::new(Trade::new(size(5), TradeDirection::Sell)),
            Box::new(Trade::new(size(10), TradeDirection::Buy)),
        ]);
        
        let model = CLVRModel::new(size(100), size(100));

        let p_0 = U256::from(size(1));
        
        model.clvr_order(p_0, &mut mock_omega);
        
        assert!(mock_omega == expected);
    }
}