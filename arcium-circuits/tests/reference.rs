use dark_vault_circuits::{
    AllocationError, AllocationInput, BPS_DENOMINATOR, PoolParams, compute_allocation,
};

fn case(seed: u16) -> AllocationInput {
    let first_cap = 3_000 + (seed % 11) * 100;
    let second_cap = 2_500 + (seed % 7) * 100;
    AllocationInput {
        total_capital: 100_000 + u64::from(seed) * 10_000,
        pools: [
            PoolParams {
                yield_bps: 700 + seed * 3,
                risk_score: 200 + seed,
                cap_bps: first_cap,
            },
            PoolParams {
                yield_bps: 600 + seed * 2,
                risk_score: 280 + seed,
                cap_bps: second_cap,
            },
            PoolParams {
                yield_bps: 400 + seed,
                risk_score: 450 + seed,
                cap_bps: 10_000,
            },
        ],
    }
}

#[test]
fn twenty_four_non_degenerate_fixtures_preserve_invariants() {
    for seed in 1..=24 {
        let fixture = case(seed);
        assert!(fixture.total_capital > 0);
        assert!(
            fixture
                .pools
                .windows(2)
                .all(|pair| pair[0].yield_bps != pair[1].yield_bps)
        );
        let output = compute_allocation(&fixture).expect("valid fixture");
        assert_eq!(
            output.weights_bps.iter().copied().sum::<u16>(),
            BPS_DENOMINATOR
        );
        for (weight, pool) in output.weights_bps.iter().zip(fixture.pools) {
            assert!(*weight <= pool.cap_bps);
        }
    }
}

#[test]
fn cap_binding_and_ties_are_deterministic() {
    let input = AllocationInput {
        total_capital: 1,
        pools: [
            PoolParams {
                yield_bps: 500,
                risk_score: 100,
                cap_bps: 4_000,
            },
            PoolParams {
                yield_bps: 500,
                risk_score: 100,
                cap_bps: 4_000,
            },
            PoolParams {
                yield_bps: 100,
                risk_score: 100,
                cap_bps: 4_000,
            },
        ],
    };
    let output = compute_allocation(&input).expect("valid");
    assert_eq!(output.weights_bps, [4_000, 4_000, 2_000]);
}

#[test]
fn single_pool_dominance_respects_cap() {
    let mut input = case(2);
    input.pools[0] = PoolParams {
        yield_bps: 5_000,
        risk_score: 1,
        cap_bps: 7_500,
    };
    let output = compute_allocation(&input).expect("valid");
    assert_eq!(output.weights_bps[0], 7_500);
}

#[test]
fn degenerate_inputs_are_rejected() {
    let mut input = case(1);
    input.total_capital = 0;
    assert_eq!(
        compute_allocation(&input),
        Err(AllocationError::ZeroCapital)
    );

    let mut input = case(1);
    input.pools[1].risk_score = 0;
    assert_eq!(
        compute_allocation(&input),
        Err(AllocationError::ZeroRiskScore { pool: 1 })
    );

    let input = AllocationInput {
        total_capital: 1,
        pools: [PoolParams {
            yield_bps: 1,
            risk_score: 1,
            cap_bps: 3_000,
        }; 3],
    };
    assert_eq!(
        compute_allocation(&input),
        Err(AllocationError::InsufficientCapacity)
    );
}
