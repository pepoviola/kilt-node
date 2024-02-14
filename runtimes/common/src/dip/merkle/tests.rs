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

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use did::did_details::DidVerificationKey;
use frame_support::assert_noop;

use crate::{
	constants::dip_provider::MAX_LINKED_ACCOUNTS,
	dip::{
		merkle::{DidMerkleProofError, DidMerkleRootGenerator},
		mock::{create_linked_info, ExtBuilder, TestRuntime, ACCOUNT},
	},
};

#[test]
fn generate_commitment_unsupported_version() {
	let linked_info = create_linked_info::<MAX_LINKED_ACCOUNTS>(DidVerificationKey::Account(ACCOUNT), true);
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			DidMerkleRootGenerator::<TestRuntime>::generate_proof(
				&linked_info,
				1,
				[].into_iter(),
				false,
				[].into_iter()
			),
			DidMerkleProofError::UnsupportedVersion
		);
	});
}
