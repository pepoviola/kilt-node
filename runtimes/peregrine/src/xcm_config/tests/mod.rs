// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod peregrine;
mod rococo;

use peregrine::Runtime as PeregrineRuntime;
use polkadot_primitives::{AccountId, Balance};
use rococo::Runtime as RococoRuntime;
use sp_core::{sr25519, Get};
use xcm::prelude::*;
use xcm_emulator::{decl_test_networks, BridgeMessageHandler, Parachain, RelayChain, TestExt};
use xcm_executor::traits::ConvertLocation;

decl_test_networks! {
	pub struct RococoNetwork {
		relay_chain = RococoRuntime,
		parachains = vec![
			PeregrineRuntime,
		],
		bridge = ()
	}
}
