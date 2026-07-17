//! Deterministic mock pool for accounting and coordinator tests. It performs no CPI.

use anchor_lang::prelude::*;

use crate::{adapters::PoolAdapter, errors::VaultError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MockPool {
    pub position: u64,
    pub yield_bps: u16,
}

impl PoolAdapter for MockPool {
    fn deposit(&mut self, amount: u64) -> Result<()> {
        self.position = self
            .position
            .checked_add(amount)
            .ok_or(VaultError::ArithmeticOverflow)?;
        Ok(())
    }

    fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.position = self
            .position
            .checked_sub(amount)
            .ok_or(VaultError::ArithmeticOverflow)?;
        Ok(())
    }

    fn read_position(&self) -> u64 {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_pool_rejects_underflow() {
        let mut pool = MockPool::default();
        assert!(pool.withdraw(1).is_err());
        pool.deposit(7).expect("deposit");
        pool.withdraw(3).expect("withdraw");
        assert_eq!(pool.read_position(), 4);
    }
}
