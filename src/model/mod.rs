use alloy::primitives::U256;
use crate::trade::{Trade, TradeDirection};
use std::ops::Index;

pub mod clvr_model;

// Notation for a particular trades ordering.
pub struct Omega(Vec<Box<dyn Trade>>);

impl Omega {
    pub fn new() -> Self {
        Omega(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.0.swap(index1, index2);
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