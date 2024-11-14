// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(all(not(feature = "std"), not(feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

/// The storage macro allows this struct to be used in persistent
/// storage. It accepts fields that implement the StorageType trait. Built-in
/// storage types for Solidity ABI primitives are found under
/// stylus_sdk::storage.
#[storage]
/// The entrypoint macro defines where Stylus execution begins. External methods
/// are exposed by annotating an impl for this struct with #[external] as seen
/// below.
#[entrypoint]
pub struct Counter {
    number: StorageU256,
}

/// Declare that [`Counter`] is a contract with the following external methods.
#[public]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[motsu::test]
    fn it_gets_number(contract: Counter) {
        let number = contract.number();
        assert_eq!(U256::ZERO, number);
    }

    #[motsu::test]
    fn it_sets_number(contract: Counter) {
        contract.set_number(U256::from(5));
        let number = contract.number();
        assert_eq!(U256::from(5), number);
    }
}
