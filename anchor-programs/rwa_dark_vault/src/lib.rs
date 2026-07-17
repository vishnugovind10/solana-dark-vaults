//! Anchor settlement and accounting layer for the dark-vault reference architecture.
#![allow(ambiguous_glob_reexports, unexpected_cfgs)]
#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::wildcard_imports
)]

use anchor_lang::prelude::*;

pub mod adapters;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqkZJY2RwrqE9g6L9LqTnX6A");

#[program]
pub mod rwa_dark_vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        callback_authority: Pubkey,
        pool_registry: [Pubkey; 3],
    ) -> Result<()> {
        instructions::initialize::handler(ctx, callback_authority, pool_registry)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, shares)
    }

    pub fn queue_rebalance(ctx: Context<QueueRebalance>, input_commitment: [u8; 32]) -> Result<()> {
        instructions::queue_rebalance::handler(ctx, input_commitment)
    }

    pub fn settle_rebalance(
        ctx: Context<SettleRebalance>,
        computation_epoch: u64,
        weights_bps: [u16; 3],
    ) -> Result<()> {
        instructions::settle_rebalance::handler(ctx, computation_epoch, weights_bps)
    }

    pub fn set_paused(ctx: Context<Admin>, paused: bool) -> Result<()> {
        instructions::admin::handler(ctx, paused)
    }
}
