use eyre::Ok;
#![cfg_attr(not(feature = "export-abi", no_main))]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
        address owner;
    }
}

#[external]
impl Counter {
    /// Returns the current number stored in the contract.
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    /// Sets the number stored in the contract.
    /// Only the contract owner can call this function.
    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.assert_owner()?;
        self.number.set(new_number);
        Ok(())
    }

    /// Increments the number stored in the contract by 1.
    pub fn increment(&mut self) -> Result<(), Vec<u8>> {
        let number: U256 = self.number.get();
        self.number.set(number + U256::from(1));
        Ok(())
    }

    /// Decrements the number stored in the contract by 1.
    pub fn decrement(&mut self) -> Result<(), Vec<u8>> {
        let number: U256 = self.number.get();
        if number > U256::from(0) {
            self.number.set(number - U256::from(1));
            Ok(())
        } else {
            Err(b"Cannot decrement below 0".to_vec())
        }
    }

    /// Initializes the contract with the given number and owner.
    pub fn init(&mut self, number: U256, owner: Address) -> Result<(), Vec<u8>> {
        self.number.set(number);
        self.owner.set(owner);
        Ok(())
    }

    /// Asserts that the caller is the contract owner.
    fn assert_owner(&self) -> Result<(), Vec<u8>> {
        if self.owner.get() == msg.sender() {
            Ok(())
        } else {
            Err(b"Only the contract owner can call this function".to_vec())
        }
    }
}