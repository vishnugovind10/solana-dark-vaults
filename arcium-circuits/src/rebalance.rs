use core::fmt;

pub const POOL_COUNT: usize = 3;
pub const BPS_DENOMINATOR: u16 = 10_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PoolParams {
    pub yield_bps: u16,
    pub risk_score: u16,
    pub cap_bps: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocationInput {
    pub total_capital: u64,
    pub pools: [PoolParams; POOL_COUNT],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocationOutput {
    pub weights_bps: [u16; POOL_COUNT],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationError {
    ZeroCapital,
    ZeroRiskScore { pool: usize },
    InvalidCap { pool: usize },
    InsufficientCapacity,
}

impl fmt::Display for AllocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCapital => formatter.write_str("total capital must be greater than zero"),
            Self::ZeroRiskScore { pool } => write!(formatter, "pool {pool} has zero risk score"),
            Self::InvalidCap { pool } => write!(formatter, "pool {pool} cap exceeds 10,000 bps"),
            Self::InsufficientCapacity => {
                formatter.write_str("pool caps cannot allocate 10,000 bps")
            }
        }
    }
}

impl std::error::Error for AllocationError {}

/// Ranks pools by integer risk-adjusted yield and fills caps greedily.
///
/// Ties resolve by lower pool index. This is deliberately simple and deterministic; private
/// deployments can substitute their own strategy without changing the settlement interface.
///
/// # Errors
///
/// Rejects zero capital, zero risk scores, caps above 10,000 bps, and aggregate capacity below
/// 10,000 bps.
pub fn compute_allocation(input: &AllocationInput) -> Result<AllocationOutput, AllocationError> {
    if input.total_capital == 0 {
        return Err(AllocationError::ZeroCapital);
    }

    let mut capacity = 0_u32;
    let mut scored = [(0_u32, 0_usize); POOL_COUNT];
    for (index, pool) in input.pools.iter().enumerate() {
        if pool.risk_score == 0 {
            return Err(AllocationError::ZeroRiskScore { pool: index });
        }
        if pool.cap_bps > BPS_DENOMINATOR {
            return Err(AllocationError::InvalidCap { pool: index });
        }
        capacity = capacity.saturating_add(u32::from(pool.cap_bps));
        scored[index] = (
            u32::from(pool.yield_bps) * u32::from(BPS_DENOMINATOR) / u32::from(pool.risk_score),
            index,
        );
    }
    if capacity < u32::from(BPS_DENOMINATOR) {
        return Err(AllocationError::InsufficientCapacity);
    }

    scored.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));

    let mut weights_bps = [0_u16; POOL_COUNT];
    let mut remaining = BPS_DENOMINATOR;
    for (_, index) in scored {
        let grant = input.pools[index].cap_bps.min(remaining);
        weights_bps[index] = grant;
        remaining -= grant;
    }
    debug_assert_eq!(remaining, 0);
    Ok(AllocationOutput { weights_bps })
}
