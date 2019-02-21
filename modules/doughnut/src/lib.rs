// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// #![cfg_attr(not(feature = "std"), feature(alloc))]

extern crate parity_codec as codec;
// Needed for deriving `Encode` and `Decode` for `RawEvent`.
#[macro_use]
extern crate parity_codec_derive;
extern crate sr_io as io;
extern crate sr_primitives as primitives;
// Needed for type-safe access to storage DB.
#[macro_use]
extern crate srml_support as runtime_support;
// `system` module provides us with all sorts of useful stuff and macros
// depend on it being around.
extern crate srml_system as system;
extern crate substrate_primitives;

use std::time::SystemTime;

use codec::Encode;
use primitives::traits::Verify;
use runtime_support::{dispatch::Result};
use runtime_support::rstd::prelude::*;

use cennznet_primitives::AccountId;
use cennznet_primitives::Signature;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// derive Debug to meet the requirement of deposit_event

#[derive(Debug, Encode, Decode, Clone, Eq, PartialEq, Default)]
struct Certificate {
	expires: u64,
	version: u32,
	holder: AccountId,
	not_before: Option<u64>,
	//	use vec of tuple to work as a key value map
	permissions: Vec<(Vec<u8>, Vec<u8>)>,
	issuer: AccountId,
}

#[derive(Debug, Encode, Decode, Clone, Eq, PartialEq, Default)]
pub struct Doughnut {
	certificate: Certificate,
	signature: Signature,
	compact: Vec<u8>,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event<T>() = default;


		pub fn validate(doughnut: Doughnut) -> Result {
			let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
				Ok(n) => n.as_secs(),
				Err(_) => return Err("SystemTime before UNIX EPOCH!")

			};
			if doughnut.certificate.expires > now {
				let valid = match doughnut.certificate.not_before {
					Some(not_before) => not_before <= now,
					None => true
				};
				if valid {
					if doughnut.signature.verify(doughnut.certificate.encode().as_slice(), &doughnut.certificate.issuer) {
						// TODO: ensure doughnut hasn't been revoked
//						Self::deposit_event(RawEvent::Validated(doughnut.certificate.issuer, doughnut.compact));
						return Ok(());
					} else {
						return Err("invalid signature");
					}


				}
			}
			return Err("invalid doughnut");
		}


		pub fn validate_permission(doughnut: Doughnut) -> Result {
			// not efficient, optimize later
			for permission_pair in &doughnut.certificate.permissions {
				if permission_pair.0 == "cennznet".encode() {
					return Ok(())
				}
			}
			return Err("no permission")
		}
	}
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId  {
		Validated(AccountId, Vec<u8>),
	}
);
