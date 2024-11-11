use crate::model::{Model, Omega};
use crate::trade::Trade;
use alloy::{primitives::U256, signers::k256::elliptic_curve::consts::U2};
use crate::model::clvr_model::CLVRModel;
use rug::{Float, Integer};

fn ln(x: U256) -> U256 {
    let int = Integer::from_str_radix(&x.to_string(), 10).expect("Failed to convert U256 to Integer");
    let ln_res = Float::with_val(256, &int).ln();

    U256::from_str_radix(&ln_res.to_string(), 10).expect("Failed to convert Float to U256")
}

impl CLVRModel {
    fn clvr_order(
        &self,
        p_0: U256,
        omega: &mut Omega,
    ) {
        let size = omega.len();
        let ln_p0 = ln(p_0);
        let two = U256::from(2);
        
        // think of this as a selection sort algorithm
        for t in 0..size {
            // select t'th trade by minimizing ( ln(p_0) - ln(P(o, t)) )^2
            let mut candidate_index = t;
            let mut candidate_value = (ln_p0 - ln(self.P(omega, t))).pow(two);
    
            for i in t..size {
                let value = (ln_p0 - ln(self.P(omega, i))).pow(two);
    
                if value < candidate_value {
                    candidate_index = i;
                    candidate_value = value;
                }
            }

            omega.swap(t, candidate_index);
        }
    }
}
