use anchor_lang::prelude::*;

use crate::state::Vault;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
    let _returned_assets = ctx.accounts.vault.record_withdrawal(shares)?;
    Ok(())
}
