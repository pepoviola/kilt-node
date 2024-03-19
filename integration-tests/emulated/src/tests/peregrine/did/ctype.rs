use crate::{
	mock::{
		network::MockNetworkRococo,
		para_chains::{AssetHubRococo, AssetHubRococoPallet, Peregrine},
		relay_chains::Rococo,
	},
	tests::peregrine::did::utils::{
		construct_xcm_message, create_mock_did, get_asset_hub_sovereign_account, get_sibling_destination_peregrine,
	},
};
use frame_support::{assert_ok, traits::fungible::Mutate};
use parity_scale_codec::Encode;
use rococo_runtime::System as RococoSystem;
use runtime_common::{constants::KILT, AccountId, Balance};
use xcm::{DoubleEncoded, VersionedXcm};
use xcm_emulator::{assert_expected_events, OriginKind, Parachain, TestExt};

fn get_xcm_message_ctype_creation(origin_kind: OriginKind, withdraw_balance: Balance) -> VersionedXcm<()> {
	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	let call: DoubleEncoded<()> = <Peregrine as Parachain>::RuntimeCall::Did(did::Call::dispatch_as {
		did_identifier: asset_hub_sovereign_account,
		call: Box::new(<Peregrine as Parachain>::RuntimeCall::Ctype(ctype::Call::add {
			ctype: b"{\"foo\": \"bar\"}".to_vec(),
		})),
	})
	.encode()
	.into();

	construct_xcm_message(origin_kind, withdraw_balance, call)
}

#[test]
fn test_ctype_creation_from_asset_hub_successful() {
	MockNetworkRococo::reset();

	let sudo_origin = <AssetHubRococo as Parachain>::RuntimeOrigin::root();

	let init_balance = KILT * 10;

	let xcm_ctype_call = get_xcm_message_ctype_creation(OriginKind::SovereignAccount, KILT);
	let destination = get_sibling_destination_peregrine();

	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	Peregrine::execute_with(|| {
		create_mock_did();
		<peregrine_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
	});

	AssetHubRococo::execute_with(|| {
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::send(
			sudo_origin,
			Box::new(destination),
			Box::new(xcm_ctype_call)
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
				PeregrineRuntimeEvent::Did(did::Event::DidCallDispatched(account, result)) => {
					account: account == &asset_hub_sovereign_account,
					result: result.is_ok(),
				},
				PeregrineRuntimeEvent::Ctype(ctype::Event::CTypeCreated(account, _)) => {
					account: account == &asset_hub_sovereign_account,
				},
			]
		);
	});

	Rococo::execute_with(|| {
		assert_eq!(RococoSystem::events().len(), 0);
	});
}

#[test]
fn test_ctype_creation_from_asset_hub_unsuccessful() {
	let sudo_origin = <AssetHubRococo as Parachain>::RuntimeOrigin::root();

	let init_balance = KILT * 10;

	let origin_kind_list = vec![OriginKind::Native, OriginKind::Superuser, OriginKind::Xcm];

	let destination = get_sibling_destination_peregrine();

	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	for origin_kind in origin_kind_list {
		MockNetworkRococo::reset();

		Peregrine::execute_with(|| {
			create_mock_did();
			<peregrine_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
		});

		let xcm_ctype_call = get_xcm_message_ctype_creation(origin_kind, KILT);

		AssetHubRococo::execute_with(|| {
			assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::send(
				sudo_origin.clone(),
				Box::new(destination.clone()),
				Box::new(xcm_ctype_call)
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
				PeregrineRuntimeEvent::Ctype(ctype::Event::CTypeCreated(_, _)) => true,
				_ => false,
			});

			assert!(!is_event_present);
		});

		Rococo::execute_with(|| {
			assert_eq!(RococoSystem::events().len(), 0);
		});
	}
}
