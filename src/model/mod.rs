use alloy::primitives::U256;
use crate::trade::{Trade, TradeDirection};
use std::ops::Index;

mod clvr_model;

// Notation for a particular trades ordering.
pub struct Omega(Vec<Box<dyn Trade>>);

impl Omega {
    pub fn new() -> Self {
        Omega(Vec::new())
    }
}

impl Index<usize> for Omega {
    type Output = Box<dyn Trade>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

pub trait Model {
    fn y_out(&self, o: &Omega, i: usize) -> U256;
    fn x_out(&self, o: &Omega, i: usize) -> U256;

    fn Y(&self, o: &Omega, i: usize) -> U256;
    fn X(&self, o: &Omega, i: usize) -> U256;

    fn P(&self, o: &Omega, i: usize) -> U256;
}