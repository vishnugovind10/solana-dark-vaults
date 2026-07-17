//! Arcis 0.13.2 circuit definition. Compile through `arcium build --features arcis-integration`.

use arcis::*;

#[encrypted]
mod confidential_allocation {
    use arcis::*;

    const POOL_COUNT: usize = 3;
    const BPS_DENOMINATOR: u16 = 10_000;

    #[derive(Copy, Clone)]
    pub struct PoolParams {
        pub yield_bps: u16,
        pub risk_score: u16,
        pub cap_bps: u16,
    }

    #[derive(Copy, Clone)]
    pub struct AllocationInput {
        pub total_capital: u64,
        pub pools: [PoolParams; POOL_COUNT],
    }

    #[derive(Copy, Clone)]
    pub struct AllocationOutput {
        pub weights_bps: [u16; POOL_COUNT],
    }

    #[instruction]
    pub fn compute_allocation(
        input_ctxt: Enc<Shared, AllocationInput>,
    ) -> Enc<Shared, AllocationOutput> {
        let input = input_ctxt.to_arcis();
        let mut scores = [0_u32; POOL_COUNT];
        let mut caps = [0_u16; POOL_COUNT];
        for i in 0..POOL_COUNT {
            scores[i] = input.pools[i].yield_bps as u32 * BPS_DENOMINATOR as u32
                / input.pools[i].risk_score as u32;
            caps[i] = input.pools[i].cap_bps;
        }

        let mut weights = [0_u16; POOL_COUNT];
        let mut remaining = BPS_DENOMINATOR;
        for _round in 0..POOL_COUNT {
            let mut best_index = 0_usize;
            let mut best_score = scores[0];
            for i in 1..POOL_COUNT {
                if scores[i] > best_score {
                    best_score = scores[i];
                    best_index = i;
                }
            }
            let grant = if caps[best_index] < remaining {
                caps[best_index]
            } else {
                remaining
            };
            weights[best_index] = grant;
            remaining -= grant;
            scores[best_index] = 0;
        }

        input_ctxt.owner.from_arcis(AllocationOutput {
            weights_bps: weights,
        })
    }
}
