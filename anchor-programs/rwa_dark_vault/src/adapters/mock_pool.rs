//! Deterministic mock pool for accounting and coordinator tests. It performs no CPI.

use anchor_lang::prelude::*;

use crate::{adapters::PoolAdapter, errors::VaultError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MockPool {
    pub position: u64,
    pub yield_bps: u16,
}

impl PoolAdapter for MockPool {
    fn deposit(&mut self, amount: u64) -> Result<()> {
        self.position = self
            .position
            .checked_add(amount)
            .ok_or(VaultError::ArithmeticOverflow)?;
        Ok(())
    }

    fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.position = self
            .position
            .checked_sub(amount)
            .ok_or(VaultError::ArithmeticOverflow)?;
        Ok(())
    }

    fn read_position(&self) -> u64 {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_pool_rejects_underflow() {
        let mut pool = MockPool::default();
        assert!(pool.withdraw(1).is_err());
        pool.deposit(7).expect("deposit");
        pool.withdraw(3).expect("withdraw");
        assert_eq!(pool.read_position(), 4);
    }

    #[test]
    fn test_mock_pool_table_driven() {
        enum Op {
            Deposit(u64),
            Withdraw(u64),
        }

        struct TestCase {
            name: &'static str,
            initial_position: u64,
            op: Op,
            expect_success: bool,
            expected_position: u64,
            expected_error_msg: Option<&'static str>,
        }

        let cases = vec![
            TestCase {
                name: "deposit zero",
                initial_position: 100,
                op: Op::Deposit(0),
                expect_success: true,
                expected_position: 100,
                expected_error_msg: None,
            },
            TestCase {
                name: "withdraw zero",
                initial_position: 100,
                op: Op::Withdraw(0),
                expect_success: true,
                expected_position: 100,
                expected_error_msg: None,
            },
            TestCase {
                name: "deposit max safe value",
                initial_position: 0,
                op: Op::Deposit(u64::MAX),
                expect_success: true,
                expected_position: u64::MAX,
                expected_error_msg: None,
            },
            TestCase {
                name: "withdraw exact balance",
                initial_position: 500,
                op: Op::Withdraw(500),
                expect_success: true,
                expected_position: 0,
                expected_error_msg: None,
            },
            TestCase {
                name: "deposit overflow keeps state unchanged",
                initial_position: u64::MAX,
                op: Op::Deposit(1),
                expect_success: false,
                expected_position: u64::MAX,
                expected_error_msg: Some("arithmetic operation overflowed or underflowed"),
            },
            TestCase {
                name: "withdraw underflow keeps state unchanged",
                initial_position: 10,
                op: Op::Withdraw(11),
                expect_success: false,
                expected_position: 10,
                expected_error_msg: Some("arithmetic operation overflowed or underflowed"),
            },
            TestCase {
                name: "large deposit overflow keeps state unchanged",
                initial_position: u64::MAX - 5,
                op: Op::Deposit(10),
                expect_success: false,
                expected_position: u64::MAX - 5,
                expected_error_msg: Some("arithmetic operation overflowed or underflowed"),
            },
            TestCase {
                name: "withdraw one-unit underflow",
                initial_position: 0,
                op: Op::Withdraw(1),
                expect_success: false,
                expected_position: 0,
                expected_error_msg: Some("arithmetic operation overflowed or underflowed"),
            },
        ];

        for case in cases {
            let mut pool = MockPool {
                position: case.initial_position,
                yield_bps: 0,
            };
            let res = match case.op {
                Op::Deposit(amt) => pool.deposit(amt),
                Op::Withdraw(amt) => pool.withdraw(amt),
            };

            if case.expect_success {
                assert!(res.is_ok(), "case `{}` failed when it should succeed: {:?}", case.name, res);
            } else {
                assert!(res.is_err(), "case `{}` succeeded when it should fail: position={}", case.name, pool.position);
                if let Some(expected_msg) = case.expected_error_msg {
                    match res.unwrap_err() {
                        anchor_lang::prelude::Error::AnchorError(ae) => {
                            assert_eq!(ae.error_msg, expected_msg, "case `{}` got wrong error message", case.name);
                        }
                        other => panic!("case `{}` expected AnchorError, got {:?}", case.name, other),
                    }
                }
            }
            assert_eq!(pool.position, case.expected_position, "case `{}` state changed unexpectedly", case.name);
        }
    }
}
