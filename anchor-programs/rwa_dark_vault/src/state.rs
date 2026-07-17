use anchor_lang::prelude::*;

use crate::errors::VaultError;

pub const VAULT_SEED: &[u8] = b"vault";
pub const POOL_COUNT: usize = 3;
pub const BPS_DENOMINATOR: u32 = 10_000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq)]
pub enum VaultStatus {
    Active,
    Paused,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct PendingComputation {
    pub active: bool,
    pub epoch: u64,
    pub input_commitment: [u8; 32],
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub authority: Pubkey,
    pub asset_mint: Pubkey,
    pub callback_authority: Pubkey,
    pub total_assets: u64,
    pub total_shares: u64,
    pub epoch: u64,
    pub status: VaultStatus,
    pub pool_registry: [Pubkey; POOL_COUNT],
    pub pending_computation: PendingComputation,
    pub last_weights_bps: [u16; POOL_COUNT],
    pub bump: u8,
}

impl Vault {
    pub fn assert_active(&self) -> Result<()> {
        require!(self.status == VaultStatus::Active, VaultError::VaultPaused);
        Ok(())
    }

    /// Mints shares with floor rounding. The first deposit mints one share per asset unit.
    pub fn record_deposit(&mut self, amount: u64) -> Result<u64> {
        self.assert_active()?;
        require!(amount > 0, VaultError::ZeroAmount);
        let shares = if self.total_shares == 0 {
            amount
        } else {
            require!(self.total_assets > 0, VaultError::ArithmeticOverflow);
            let numerator = u128::from(amount)
                .checked_mul(u128::from(self.total_shares))
                .ok_or(VaultError::ArithmeticOverflow)?;
            u64::try_from(numerator / u128::from(self.total_assets))
                .map_err(|_| VaultError::ArithmeticOverflow)?
        };
        require!(shares > 0, VaultError::DepositTooSmall);
        self.total_assets = self
            .total_assets
            .checked_add(amount)
            .ok_or(VaultError::ArithmeticOverflow)?;
        self.total_shares = self
            .total_shares
            .checked_add(shares)
            .ok_or(VaultError::ArithmeticOverflow)?;
        Ok(shares)
    }

    /// Burns shares with floor rounding so withdrawals cannot overdraw the vault.
    pub fn record_withdrawal(&mut self, shares: u64) -> Result<u64> {
        self.assert_active()?;
        require!(shares > 0, VaultError::ZeroAmount);
        require!(shares <= self.total_shares, VaultError::InsufficientShares);
        let numerator = u128::from(shares)
            .checked_mul(u128::from(self.total_assets))
            .ok_or(VaultError::ArithmeticOverflow)?;
        let amount = u64::try_from(numerator / u128::from(self.total_shares))
            .map_err(|_| VaultError::ArithmeticOverflow)?;
        require!(amount > 0, VaultError::WithdrawalTooSmall);
        self.total_shares = self
            .total_shares
            .checked_sub(shares)
            .ok_or(VaultError::ArithmeticOverflow)?;
        self.total_assets = self
            .total_assets
            .checked_sub(amount)
            .ok_or(VaultError::ArithmeticOverflow)?;
        Ok(amount)
    }

    pub fn queue_rebalance(&mut self, input_commitment: [u8; 32]) -> Result<u64> {
        self.assert_active()?;
        require!(
            !self.pending_computation.active,
            VaultError::ComputationAlreadyPending
        );
        let next_epoch = self
            .epoch
            .checked_add(1)
            .ok_or(VaultError::ArithmeticOverflow)?;
        self.pending_computation = PendingComputation {
            active: true,
            epoch: next_epoch,
            input_commitment,
        };
        Ok(next_epoch)
    }

    pub fn settle_rebalance(&mut self, epoch: u64, weights_bps: [u16; 3]) -> Result<()> {
        self.assert_active()?;
        require!(
            self.pending_computation.active,
            VaultError::NoPendingComputation
        );
        require_eq!(
            self.pending_computation.epoch,
            epoch,
            VaultError::ComputationEpochMismatch
        );
        require!(
            weights_bps
                .iter()
                .all(|weight| u32::from(*weight) <= BPS_DENOMINATOR),
            VaultError::InvalidWeight
        );
        let sum = weights_bps
            .iter()
            .try_fold(0_u32, |acc, weight| acc.checked_add(u32::from(*weight)))
            .ok_or(VaultError::ArithmeticOverflow)?;
        require_eq!(sum, BPS_DENOMINATOR, VaultError::InvalidWeightSum);
        self.epoch = epoch;
        self.last_weights_bps = weights_bps;
        self.pending_computation = PendingComputation::default();
        Ok(())
    }

    pub fn set_paused(&mut self, paused: bool) -> Result<()> {
        let target = if paused {
            VaultStatus::Paused
        } else {
            VaultStatus::Active
        };
        require!(self.status != target, VaultError::StatusUnchanged);
        self.status = target;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vault() -> Vault {
        Vault {
            authority: Pubkey::new_unique(),
            asset_mint: Pubkey::new_unique(),
            callback_authority: Pubkey::new_unique(),
            total_assets: 0,
            total_shares: 0,
            epoch: 0,
            status: VaultStatus::Active,
            pool_registry: [Pubkey::new_unique(); POOL_COUNT],
            pending_computation: PendingComputation::default(),
            last_weights_bps: [0; POOL_COUNT],
            bump: 254,
        }
    }

    #[test]
    fn deposit_and_withdraw_round_trip_is_exact() {
        let mut vault = vault();
        assert_eq!(vault.record_deposit(1_000_000).expect("deposit"), 1_000_000);
        assert_eq!(vault.record_deposit(250_000).expect("deposit"), 250_000);
        assert_eq!(vault.record_withdrawal(250_000).expect("withdraw"), 250_000);
        assert_eq!(vault.total_assets, 1_000_000);
        assert_eq!(vault.total_shares, 1_000_000);
    }

    #[test]
    fn rounding_never_overpays() {
        let mut vault = vault();
        vault.total_assets = 3;
        vault.total_shares = 2;
        assert_eq!(vault.record_deposit(2).expect("deposit"), 1);
        assert_eq!(vault.record_withdrawal(1).expect("withdraw"), 1);
        assert_eq!(vault.total_assets, 4);
    }

    #[test]
    fn pause_blocks_mutation() {
        let mut vault = vault();
        vault.set_paused(true).expect("pause");
        assert!(vault.record_deposit(1).is_err());
        assert!(vault.queue_rebalance([7; 32]).is_err());
        vault.set_paused(false).expect("unpause");
        assert!(vault.record_deposit(1).is_ok());
    }

    #[test]
    fn settlement_requires_matching_pending_epoch_and_exact_weights() {
        let mut vault = vault();
        let epoch = vault.queue_rebalance([9; 32]).expect("queue");
        assert!(
            vault
                .settle_rebalance(epoch + 1, [5_000, 3_000, 2_000])
                .is_err()
        );
        assert!(
            vault
                .settle_rebalance(epoch, [5_000, 3_000, 1_999])
                .is_err()
        );
        vault
            .settle_rebalance(epoch, [5_000, 3_000, 2_000])
            .expect("settle");
        assert_eq!(vault.epoch, 1);
        assert!(!vault.pending_computation.active);
    }
}
