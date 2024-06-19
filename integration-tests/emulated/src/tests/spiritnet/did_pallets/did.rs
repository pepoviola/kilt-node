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

use asset_hub_rococo_emulated_chain::AssetHubRococoParaPallet;
use did::did_details::DidVerificationKey;
use frame_support::{
	assert_ok,
	traits::fungible::{hold::Inspect, Mutate},
};
use parity_scale_codec::Encode;
use runtime_common::{constants::KILT, AccountId, Balance};
use xcm::{lts::prelude::OriginKind, DoubleEncoded, VersionedXcm};
use xcm_emulator::{assert_expected_events, Chain, Network, TestExt};

use crate::{
	mock::{
		network:chains:::{AssetHub, MockNetwork, Rococo, Spiritnet},
		para_chains::SpiritnetParachainParaPallet,
	},
	tests::spiritnet::did_pallets::utils::{
		construct_basic_transact_xcm_message, get_asset_hub_sovereign_account, get_sibling_destination_spiritnet,
	},
};

fn get_xcm_message_create_did(origin_kind: OriginKind, withdraw_balance: Balance) -> VersionedXcm<()> {
	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	let call: DoubleEncoded<()> = <Spiritnet as Chain>::RuntimeCall::Did(did::Call::create_from_account {
		authentication_key: DidVerificationKey::Account(asset_hub_sovereign_account),
	})
	.encode()
	.into();

	construct_basic_transact_xcm_message(origin_kind, withdraw_balance, call)
}

#[test]
fn test_did_creation_from_asset_hub_successful() {
	MockNetwork::reset();

	let sudo_origin = <AssetHub as Chain>::RuntimeOrigin::root();

	let init_balance = KILT * 10;
	let withdraw_balance = init_balance / 2;

	let xcm_create_did_msg = get_xcm_message_create_did(OriginKind::SovereignAccount, withdraw_balance);
	let destination = get_sibling_destination_spiritnet();

	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	Spiritnet::execute_with(|| {
		<spiritnet_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
	});

	AssetHub::execute_with(|| {
		assert_ok!(<AssetHub as AssetHubRococoParaPallet>::PolkadotXcm::send(
			sudo_origin,
			Box::new(destination.clone()),
			Box::new(xcm_create_did_msg.clone())
		));

		type RuntimeEvent = <AssetHub as Chain>::RuntimeEvent;
		assert_expected_events!(
			AssetHub,
			vec![
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	Spiritnet::execute_with(|| {
		type SpiritnetRuntimeEvent = <Spiritnet as Chain>::RuntimeEvent;
		assert_expected_events!(
			Spiritnet,
			vec![
				SpiritnetRuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: true, .. }) => {},
				SpiritnetRuntimeEvent::Did(did::Event::DidCreated(account, did_identifier)) => {
					account: account == &asset_hub_sovereign_account,
					did_identifier:  did_identifier == &asset_hub_sovereign_account,
				},
			]
		);

		let balance_on_hold =
			<<Spiritnet as SpiritnetParachainParaPallet>::Balances as Inspect<AccountId>>::balance_on_hold(
				&spiritnet_runtime::RuntimeHoldReason::from(did::HoldReason::Deposit),
				&asset_hub_sovereign_account,
			);

		assert_eq!(
			balance_on_hold,
			<spiritnet_runtime::Runtime as did::Config>::BaseDeposit::get()
		);
	});

	Rococo::execute_with(|| {
		assert_eq!(Rococo::events().len(), 0);
	});
}

#[test]
fn test_did_creation_from_asset_hub_unsuccessful() {
	let sudo_origin = <AssetHub as Chain>::RuntimeOrigin::root();

	let init_balance = KILT * 100;
	let withdraw_balance = init_balance / 2;

	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();
	let destination = get_sibling_destination_spiritnet();

	let origin_kind_list = vec![OriginKind::Xcm, OriginKind::Superuser, OriginKind::Native];

	for origin in origin_kind_list {
		MockNetwork::reset();

		Spiritnet::execute_with(|| {
			<spiritnet_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
		});

		let xcm_create_did_msg = get_xcm_message_create_did(origin, withdraw_balance);

		AssetHub::execute_with(|| {
			assert_ok!(<AssetHub as AssetHubRococoParaPallet>::PolkadotXcm::send(
				sudo_origin.clone(),
				Box::new(destination.clone()),
				Box::new(xcm_create_did_msg)
			));

			type RuntimeEvent = <AssetHub as Chain>::RuntimeEvent;
			assert_expected_events!(
				AssetHub,
				vec![
					RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
				]
			);
		});

		Spiritnet::execute_with(|| {
			type SpiritnetRuntimeEvent = <Spiritnet as Chain>::RuntimeEvent;

			let is_create_event_present = Spiritnet::events().iter().any(|event| {
				matches!(
					event,
					SpiritnetRuntimeEvent::Did(did::Event::<spiritnet_runtime::Runtime>::DidCreated(_, _))
				)
			});

			assert!(!is_create_event_present);
		});

		Rococo::execute_with(|| {
			assert_eq!(Rococo::events().len(), 0);
		});
	}
}
