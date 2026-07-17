//! Pool adapter boundary. Production adapters must validate every CPI program and account.

pub mod mock_pool;

use anchor_lang::prelude::*;

pub trait PoolAdapter {
    fn deposit(&mut self, amount: u64) -> Result<()>;
    fn withdraw(&mut self, amount: u64) -> Result<()>;
    fn read_position(&self) -> u64;
}
