use anchor_lang::prelude::*;

use crate::state::Vault;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub depositor: Signer<'info>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let _minted_shares = ctx.accounts.vault.record_deposit(amount)?;
    Ok(())
}
