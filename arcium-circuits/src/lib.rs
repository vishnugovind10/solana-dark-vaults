//! Fixed-width allocation logic shared by the cleartext oracle and Arcis definition.

pub mod rebalance;

#[cfg(feature = "arcis-integration")]
pub mod arcis_rebalance;

pub use rebalance::{
    AllocationError, AllocationInput, AllocationOutput, BPS_DENOMINATOR, POOL_COUNT, PoolParams,
    compute_allocation,
};
