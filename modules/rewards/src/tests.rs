//! Tests for the module.

#![cfg(test)]

use super::*;
use runtime_io::with_externalities;

use crate::mock::{ExtBuilder, Rewards, Staking, ChargeFeeMock, SessionChangeMock, Test};
use support::{assert_err, assert_ok, StorageValue};
use staking;

#[test]
fn set_block_reward_works() {
	with_externalities(&mut ExtBuilder::default().block_reward(3).build(), || {
		assert_eq!(Rewards::block_reward(), 3);
		Rewards::set_block_reward(5);
		assert_eq!(Rewards::block_reward(), 5);
	})
}

#[test]
fn on_fee_charged_works() {
	with_externalities(&mut ExtBuilder::default().build(), || {
		assert_eq!(Rewards::session_transaction_fee(), 0);

		ChargeFeeMock::trigger_rewards_on_fee_charged(3);
		assert_eq!(Rewards::session_transaction_fee(), 3);

		ChargeFeeMock::trigger_rewards_on_fee_charged(5);
		assert_eq!(Rewards::session_transaction_fee(), 3 + 5);
	})
}

#[test]
fn on_session_change_works() {
	with_externalities(&mut ExtBuilder::default().build(), || {
		assert_eq!(Staking::current_era_reward(), 0);
		<SessionTransactionFee<Test>>::put(3);
		assert_eq!(Rewards::session_transaction_fee(), 3);

		SessionChangeMock::trigger_rewards_on_session_change();
		assert_eq!(Staking::current_era_reward(), 3);
		assert_eq!(Rewards::session_transaction_fee(), 0);
	})
}
