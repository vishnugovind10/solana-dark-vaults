use anchor_lang::prelude::*;

use crate::state::{POOL_COUNT, PendingComputation, VAULT_SEED, Vault, VaultStatus};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Vault::INIT_SPACE,
        seeds = [VAULT_SEED, asset_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    /// CHECK: The address is stored only; token-program validation belongs to the custody adapter.
    pub asset_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    callback_authority: Pubkey,
    pool_registry: [Pubkey; POOL_COUNT],
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.asset_mint = ctx.accounts.asset_mint.key();
    vault.callback_authority = callback_authority;
    vault.total_assets = 0;
    vault.total_shares = 0;
    vault.epoch = 0;
    vault.status = VaultStatus::Active;
    vault.pool_registry = pool_registry;
    vault.pending_computation = PendingComputation::default();
    vault.last_weights_bps = [0; POOL_COUNT];
    vault.bump = ctx.bumps.vault;
    Ok(())
}
