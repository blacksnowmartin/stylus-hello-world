use eyre::Ok;
#![cfg_attr(not(feature = "export-abi", no_main))]

#[global_allocator]
static ALLOC: wee_alloc:  ::wee_alloc::WeeAlloc> = wee_alloc: ::wee_alloc::Wee
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
        }
}

#[external]
impl Counter {
   pub fn number(&self) -> Result<U256, Vec<u8>> {
       Ok(self.number.get())
   }
   pub fn set_number(&mut self, number: U256) -> Result<(), Vec<u8>> {
       self.number.set(new_number);
       Ok(())
   }
   pub fn increment(&mut self)  -> Result<(), Vec<u8>> {
    let number: Uint<256, 4> = self.number.get();
    self.number.set(number + U256::from(1));
    Ok(())
   }
}