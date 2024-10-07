use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
        }
}

#[external]
impl Counter {
   pub fn number(&self) -> Result<U256, Vec<un> {
}