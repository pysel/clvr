use alloy::primitives::U256;
use crate::trade::TradeDirection;
use crate::model::{Model, Omega};

struct CLVRModel {
    reserve_x: U256,
    reserve_y: U256,
}

impl CLVRModel {
    pub fn new(reserve_x: U256, reserve_y: U256) -> Self {
        CLVRModel {
            reserve_x,
            reserve_y,
        }
    }
}

impl Model for CLVRModel {
    fn y_out(&self, o: &Omega, i: usize) -> U256 {
        if o[i].get_direction() == TradeDirection::Sell {
            let fraction = self.Y(o, i - 1) / (self.X(o, i - 1) + o[i].get_amount_in());
            return fraction * o[i].get_amount_in();
        }

        U256::from(0)
    }

    fn x_out(&self, o: &Omega, i: usize) -> U256 {
        if o[i].get_direction() == TradeDirection::Buy {
            let fraction = self.X(o, i - 1) / (self.Y(o, i - 1) + o[i].get_amount_in());
            return fraction * o[i].get_amount_in();
        }

        U256::from(0)
    }

    fn Y(&self, o: &Omega, i: usize) -> U256 {
        if i == 0 {
            return self.reserve_y
        } else if i > 0 && o[i].get_direction() == TradeDirection::Buy {
            return self.Y(o, i - 1) + o[i].get_amount_in()
        } else if i > 0 && o[i].get_direction() == TradeDirection::Sell {
            return self.Y(o, i - 1) - self.y_out(o, i)
        }
        panic!("Invalid call to Y");
    }
    
    fn X(&self, o: &Omega, i: usize) -> U256 {
        if i == 0 {
            return self.reserve_x
        } else if i > 0 && o[i].get_direction() == TradeDirection::Sell {
            return self.X(o, i - 1) + o[i].get_amount_in()
        } else if i > 0 && o[i].get_direction() == TradeDirection::Buy {
            return self.X(o, i - 1) - self.x_out(o, i)
        }
        panic!("Invalid call to X");
    }

    fn P(&self, o: &Omega, i: usize) -> U256 {
        self.Y(o, i) / self.X(o, i)
    }
}