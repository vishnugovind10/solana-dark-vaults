use anchor_lang::prelude::*;

use crate::{errors::VaultError, state::Vault};

#[derive(Accounts)]
pub struct SettleRebalance<'info> {
    #[account(
        mut,
        has_one = callback_authority @ VaultError::UnauthorizedCallback
    )]
    pub vault: Account<'info, Vault>,
    pub callback_authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<SettleRebalance>,
    computation_epoch: u64,
    weights_bps: [u16; 3],
) -> Result<()> {
    ctx.accounts
        .vault
        .settle_rebalance(computation_epoch, weights_bps)
}
