use anchor_lang::prelude::*;

use crate::{errors::VaultError, state::Vault};

#[derive(Accounts)]
pub struct QueueRebalance<'info> {
    #[account(mut, has_one = authority @ VaultError::Unauthorized)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<QueueRebalance>, input_commitment: [u8; 32]) -> Result<()> {
    let _epoch = ctx.accounts.vault.queue_rebalance(input_commitment)?;
    Ok(())
}
