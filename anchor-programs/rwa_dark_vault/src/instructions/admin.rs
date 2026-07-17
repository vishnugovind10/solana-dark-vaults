use anchor_lang::prelude::*;

use crate::{errors::VaultError, state::Vault};

#[derive(Accounts)]
pub struct Admin<'info> {
    #[account(mut, has_one = authority @ VaultError::Unauthorized)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<Admin>, paused: bool) -> Result<()> {
    ctx.accounts.vault.set_paused(paused)
}
