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

use frame_support::{assert_ok, traits::fungible::Mutate};
use parity_scale_codec::Encode;
use runtime_common::{constants::KILT, AccountId, Balance};
use sp_core::H256;
use xcm::VersionedXcm;
use xcm_emulator::{assert_expected_events, OriginKind, Parachain, TestExt};

use crate::{
	mock::{
		network::MockNetworkRococo,
		para_chains::{AssetHubRococo, AssetHubRococoPallet, Peregrine},
		relay_chains::Rococo,
	},
	tests::peregrine::did_pallets::utils::{
		construct_xcm_message, create_mock_ctype, create_mock_did, get_asset_hub_sovereign_account,
		get_sibling_destination_peregrine,
	},
};

fn get_xcm_message_attestation_creation(
	origin_kind: OriginKind,
	withdraw_balance: Balance,
	ctype_hash: H256,
	claim_hash: H256,
) -> VersionedXcm<()> {
	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	let call = <Peregrine as Parachain>::RuntimeCall::Did(did::Call::dispatch_as {
		did_identifier: asset_hub_sovereign_account,
		call: Box::new(<Peregrine as Parachain>::RuntimeCall::Attestation(
			attestation::Call::add {
				claim_hash,
				ctype_hash,
				authorization: None,
			},
		)),
	})
	.encode()
	.into();

	construct_xcm_message(origin_kind, withdraw_balance, call)
}

#[test]
fn test_attestation_creation_from_asset_hub_successful() {
	MockNetworkRococo::reset();

	let sudo_origin = <AssetHubRococo as Parachain>::RuntimeOrigin::root();

	let ctype_hash_value = H256([0; 32]);
	let claim_hash_value = H256([1; 32]);

	let init_balance = KILT * 10;
	let withdraw_balance = init_balance / 2;

	let xcm_attestation_call = get_xcm_message_attestation_creation(
		OriginKind::SovereignAccount,
		withdraw_balance,
		ctype_hash_value.clone(),
		claim_hash_value.clone(),
	);

	let destination = get_sibling_destination_peregrine();

	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	Peregrine::execute_with(|| {
		create_mock_ctype(ctype_hash_value.clone());
		create_mock_did();
		<peregrine_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
	});

	AssetHubRococo::execute_with(|| {
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::send(
			sudo_origin.clone(),
			Box::new(destination.clone()),
			Box::new(xcm_attestation_call)
		));

		type RuntimeEvent = <AssetHubRococo as Parachain>::RuntimeEvent;
		assert_expected_events!(
			AssetHubRococo,
			vec![
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	Peregrine::execute_with(|| {
		type PeregrineRuntimeEvent = <Peregrine as Parachain>::RuntimeEvent;

		assert_expected_events!(
			Peregrine,
			vec![
				PeregrineRuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. }) => {},
				PeregrineRuntimeEvent::Attestation(attestation::Event::AttestationCreated { attester, claim_hash, authorization: _ , ctype_hash }) => {
					attester: attester == &asset_hub_sovereign_account,
					claim_hash: claim_hash == &claim_hash_value,
					ctype_hash: ctype_hash == &ctype_hash_value,
				},
			]
		);
	});

	Rococo::execute_with(|| {
		assert_eq!(Rococo::events().len(), 0);
	});
}

#[test]
fn test_attestation_creation_from_asset_hub_unsuccessful() {
	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();
	let sudo_origin = <AssetHubRococo as Parachain>::RuntimeOrigin::root();
	let destination = get_sibling_destination_peregrine();

	let ctype_hash_value = H256([0; 32]);
	let claim_hash_value = H256([1; 32]);

	let init_balance = KILT * 100;
	let withdraw_balance = init_balance / 2;

	let origin_kind_list = vec![OriginKind::Native, OriginKind::Superuser, OriginKind::Xcm];

	for origin_kind in origin_kind_list {
		MockNetworkRococo::reset();

		Peregrine::execute_with(|| {
			create_mock_ctype(ctype_hash_value.clone());
			create_mock_did();
			<peregrine_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
		});

		let xcm_attestation_call = get_xcm_message_attestation_creation(
			origin_kind,
			withdraw_balance,
			ctype_hash_value.clone(),
			claim_hash_value.clone(),
		);

		AssetHubRococo::execute_with(|| {
			assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::send(
				sudo_origin.clone(),
				Box::new(destination.clone()),
				Box::new(xcm_attestation_call)
			));

			type RuntimeEvent = <AssetHubRococo as Parachain>::RuntimeEvent;
			assert_expected_events!(
				AssetHubRococo,
				vec![
					RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
				]
			);
		});

		Peregrine::execute_with(|| {
			type PeregrineRuntimeEvent = <Peregrine as Parachain>::RuntimeEvent;

			let is_event_present = Peregrine::events().iter().any(|event| match event {
				PeregrineRuntimeEvent::Did(did::Event::DidCallDispatched(_, _)) => true,
				PeregrineRuntimeEvent::Attestation(attestation::Event::AttestationCreated { .. }) => true,
				_ => false,
			});

			assert!(!is_event_present);
		});

		Rococo::execute_with(|| {
			assert_eq!(Rococo::events().len(), 0);
		});
	}
}
