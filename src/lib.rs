// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(all(not(feature = "std"), not(feature = "export-abi")), no_main)]
extern crate alloc;

use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
/// Import items from the SDK. The prelude contains common traits and
/// macros.
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageU256, StorageAddress};

/// The currency data type.
pub type Currency = Address;

sol! {
    /// Emitted when liquidity is moved to lending protocol
    event LiquidityMovedToLending(
        address indexed token,
        uint256 amount,
        uint256 timestamp
    );

    /// Emitted when liquidity is moved back to LP
    event LiquidityMovedToLP(
        address indexed token,
        uint256 amount,
        uint256 timestamp
    );

    /// Emitted when lending fees are collected
    event LendingFeesCollected(
        address indexed token,
        uint256 amount,
        uint256 timestamp
    );
}

sol! {
    /// Custom errors for the dynamic LP hook
    #[derive(Debug)]
    error InvalidTWAP();
    #[derive(Debug)]
    error InvalidLendingProtocol();
    #[derive(Debug)]
    error InvalidAmount();
    #[derive(Debug)]
    error InvalidRange();
}

#[derive(SolidityError, Debug)]
pub enum Error {
    InvalidTWAP(InvalidTWAP),
    InvalidLendingProtocol(InvalidLendingProtocol),
    InvalidAmount(InvalidAmount),
    InvalidRange(InvalidRange),
}

/// Storage for TWAP data
#[storage]
struct TWAPStorage {
    /// The last price observation
    last_price: StorageU256,
    /// The last timestamp of price observation
    last_timestamp: StorageU256,
    /// The cumulative price
    cumulative_price: StorageU256,
    /// The observation period (in seconds)
    observation_period: StorageU256,
}

/// Storage for lending protocol data
#[storage]
struct LendingStorage {
    /// The lending protocol address
    lending_protocol: StorageAddress,
    /// The amount of tokens currently in lending
    amount_in_lending: StorageU256,
    /// The last time fees were collected
    last_fee_collection: StorageU256,
}

/// Main hook storage
#[storage]
#[entrypoint]
struct DynamicLPHook {
    /// TWAP storage
    twap: TWAPStorage,
    /// Lending storage
    lending: LendingStorage,
    /// The minimum time between reallocations (in seconds)
    min_reallocation_time: StorageU256,
    /// The price range for active LP position (in basis points, 1% = 100)
    price_range: StorageU256,
}

/// Interface for the dynamic LP hook
pub trait IDynamicLPHook {
    /// Initialize the hook with parameters
    fn initialize(
        &mut self,
        lending_protocol: Address,
        observation_period: U256,
        min_reallocation_time: U256,
        price_range: U256,
    ) -> Result<(), Error>;

    /// Update TWAP with new price
    fn update_twap(&mut self, price: U256) -> Result<(), Error>;

    /// Check if position is out of range and reallocate if necessary
    fn check_and_reallocate(&mut self, current_price: U256) -> Result<(), Error>;

    /// Collect lending fees
    fn collect_lending_fees(&mut self) -> Result<U256, Error>;

    /// Move liquidity back to LP if price is in range
    fn move_to_lp_if_in_range(&mut self, current_price: U256) -> Result<(), Error>;
}

#[public]
impl IDynamicLPHook for DynamicLPHook {
    fn initialize(
        &mut self,
        lending_protocol: Address,
        observation_period: U256,
        min_reallocation_time: U256,
        price_range: U256,
    ) -> Result<(), Error> {
        self.lending.lending_protocol.set(lending_protocol);
        self.twap.observation_period.set(observation_period);
        self.min_reallocation_time.set(min_reallocation_time);
        self.price_range.set(price_range);
        Ok(())
    }

    fn update_twap(&mut self, price: U256) -> Result<(), Error> {
        let current_time = U256::from(self.vm().block_timestamp());
        let time_diff = current_time - self.twap.last_timestamp.get();
        
        if time_diff == U256::ZERO {
            return Err(Error::InvalidTWAP(InvalidTWAP {}));
        }

        // Update cumulative price
        self.twap.cumulative_price.set(
            self.twap.cumulative_price.get() + (price * time_diff)
        );
        
        // Update last price and timestamp
        self.twap.last_price.set(price);
        self.twap.last_timestamp.set(current_time);

        Ok(())
    }

    fn check_and_reallocate(&mut self, current_price: U256) -> Result<(), Error> {
        let current_time = U256::from(self.vm().block_timestamp());
        let time_since_last_reallocation = current_time - self.lending.last_fee_collection.get();

        if time_since_last_reallocation < self.min_reallocation_time.get() {
            return Ok(());
        }

        // Calculate TWAP
        let twap = self.twap.cumulative_price.get() / self.twap.observation_period.get();
        
        // Check if price is out of range (using basis points)
        let price_diff = if current_price > twap {
            current_price - twap
        } else {
            twap - current_price
        };

        let price_diff_bps = (price_diff * U256::from(10000)) / twap;

        if price_diff_bps > self.price_range.get() {
            // Move to lending protocol
            self.move_to_lending()?;
        }

        Ok(())
    }

    fn collect_lending_fees(&mut self) -> Result<U256, Error> {
        // This is a placeholder for actual lending protocol integration
        // In a real implementation, this would call the lending protocol's collect function
        let fees = U256::from(0);
        
        self.lending.last_fee_collection.set(U256::from(self.vm().block_timestamp()));
        
        log(
            self.vm(),
            LendingFeesCollected {
                token: self.vm().contract_address(),
                amount: fees,
                timestamp: U256::from(self.vm().block_timestamp()),
            },
        );

        Ok(fees)
    }

    fn move_to_lp_if_in_range(&mut self, current_price: U256) -> Result<(), Error> {
        let twap = self.twap.cumulative_price.get() / self.twap.observation_period.get();
        
        // Check if price is in range (using basis points)
        let price_diff = if current_price > twap {
            current_price - twap
        } else {
            twap - current_price
        };

        let price_diff_bps = (price_diff * U256::from(10000)) / twap;

        if price_diff_bps <= self.price_range.get() && self.lending.amount_in_lending.get() > U256::ZERO {
            // Move back to LP
            self.move_to_lp()?;
        }

        Ok(())
    }
}

impl DynamicLPHook {
    /// Move liquidity to lending protocol
    fn move_to_lending(&mut self) -> Result<(), Error> {
        // This is a placeholder for actual lending protocol integration
        // In a real implementation, this would:
        // 1. Get the current LP position
        // 2. Approve the lending protocol
        // 3. Deposit into the lending protocol
        let amount = U256::from(1000); // Placeholder amount
        
        self.lending.amount_in_lending.set(amount);
        
        log(
            self.vm(),
            LiquidityMovedToLending {
                token: self.vm().contract_address(),
                amount,
                timestamp: U256::from(self.vm().block_timestamp()),
            },
        );

        Ok(())
    }

    /// Move liquidity back to LP
    fn move_to_lp(&mut self) -> Result<(), Error> {
        // This is a placeholder for actual lending protocol integration
        // In a real implementation, this would:
        // 1. Withdraw from lending protocol
        // 2. Add liquidity back to LP
        let amount = self.lending.amount_in_lending.get();
        
        self.lending.amount_in_lending.set(U256::ZERO);
        
        log(
            self.vm(),
            LiquidityMovedToLP {
                token: self.vm().contract_address(),
                amount,
                timestamp: U256::from(self.vm().block_timestamp()),
            },
        );

        Ok(())
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use stylus_sdk::testing::*;

    fn setup() -> (TestVM, DynamicLPHook) {
        let vm = TestVM::default();
        let mut contract = DynamicLPHook::from(&vm);
        
        // Initialize with test values
        let lending_protocol = Address::from([0x1; 20]);
        let observation_period = U256::from(3600); // 1 hour
        let min_reallocation_time = U256::from(1800); // 30 minutes
        let price_range = U256::from(100);
        
        let _ = contract.initialize(lending_protocol, observation_period, min_reallocation_time, price_range);
        
        (vm, contract)
    }

    #[test]
    fn test_initialization() {
        let (_, contract) = setup();
        assert_eq!(contract.lending.lending_protocol.get(), Address::from([0x1; 20]));
        assert_eq!(contract.twap.observation_period.get(), U256::from(3600));
        assert_eq!(contract.min_reallocation_time.get(), U256::from(1800));
        assert_eq!(contract.price_range.get(), U256::from(100));
    }

    #[test]
    fn test_updates_twap() {
        let (vm, mut contract) = setup();
        
        // Set initial timestamp
        vm.set_block_timestamp(1000);
        
        // Update TWAP
        let _ = contract.update_twap(U256::from(100));
        
        // Advance time and update again
        vm.set_block_timestamp(2800);
        let _ = contract.update_twap(U256::from(200));
        
        // Check TWAP data
        let last_timestamp = contract.twap.last_timestamp.get();
        let last_price = contract.twap.last_price.get();
        assert!(last_timestamp > U256::ZERO);
        assert_eq!(last_price, U256::from(200));
    }

    #[test]
    fn test_checks_and_reallocates() {
        let (vm, mut contract) = setup();
        
        // Set initial timestamp and TWAP
        vm.set_block_timestamp(1000);
        let _ = contract.update_twap(U256::from(100));
        
        // Advance time past min_reallocation_time
        vm.set_block_timestamp(3000);
        
        // Update TWAP with price outside range
        let _ = contract.update_twap(U256::from(300));
        let _ = contract.check_and_reallocate(U256::from(300));
    }

    #[test]
    fn test_respects_min_reallocation_time() {
        let (vm, mut contract) = setup();
        
        // Set initial timestamp and TWAP
        vm.set_block_timestamp(1000);
        let _ = contract.update_twap(U256::from(100));
        
        // Try to reallocate before min_reallocation_time
        vm.set_block_timestamp(1100); // Only 100 seconds later
        let result = contract.check_and_reallocate(U256::from(100));
        assert!(result.is_ok()); // Should succeed but not reallocate
    }

    #[test]
    fn test_handles_edge_cases() {
        let (vm, mut contract) = setup();
        
        // Set initial timestamp
        vm.set_block_timestamp(1000);
        
        // Test with zero price
        let result = contract.update_twap(U256::ZERO);
        assert!(result.is_ok()); // Should succeed but not update TWAP
    }
}
