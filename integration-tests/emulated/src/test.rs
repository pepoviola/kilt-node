use crate::{
	mock::{
		network::MockNetwork,
		para_chains::{spiritnet, AssetHub, AssetHubPallet, Spiritnet},
		relay_chains::{Polkadot, PolkadotPallet},
	},
	utils::get_account_id_from_seed,
};
use asset_hub_polkadot_runtime::PolkadotXcm as AssetHubXcm;
use asset_hub_polkadot_runtime::System as AssetHubSystem;
use frame_support::dispatch::RawOrigin;
use frame_support::{assert_err, assert_ok};
use integration_tests_common::{asset_hub_polkadot, polkadot::ED, ALICE, BOB};
use parity_scale_codec::Encode;
use polkadot_runtime::System as PolkadotSystem;
use sp_core::sr25519;
use sp_runtime::{DispatchError, ModuleError};
use spiritnet_runtime::PolkadotXcm as SpiritnetXcm;
use xcm::{v3::WeightLimit, DoubleEncoded, VersionedMultiLocation, VersionedXcm};
use xcm_emulator::{
	assert_expected_events,
	cumulus_pallet_xcmp_queue::Event as XcmpQueueEvent,
	Here,
	Instruction::{Transact, UnpaidExecution},
	Junction, Junctions, OriginKind, Parachain, Parent, ParentThen, RelayChain, TestExt, Weight, Xcm, X1,
};

/// Test that a reserved transfer to the relaychain is failing. We don't want to
/// allow transfers to the relaychain since the funds might be lost.
#[test]
fn test_reserve_asset_transfer_from_regular_spiritnet_account_to_relay() {
	MockNetwork::reset();

	let alice_account_id_on_peregrine = get_account_id_from_seed::<sr25519::Public>(ALICE);

	Spiritnet::execute_with(|| {
		assert_ok!(SpiritnetXcm::limited_reserve_transfer_assets(
			RawOrigin::Signed(alice_account_id_on_peregrine.clone()).into(),
			Box::new(Parent.into()),
			Box::new(
				X1(Junction::AccountId32 {
					network: None,
					id: alice_account_id_on_peregrine.into()
				})
				.into()
			),
			Box::new((Here, 1_000_000).into()),
			0,
			WeightLimit::Unlimited,
		));

		type RuntimeEvent = <Spiritnet as Parachain>::RuntimeEvent;

		assert_expected_events!(
			Spiritnet,
			vec![RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Attempted {
				outcome: xcm::latest::Outcome::Error(xcm::latest::Error::Barrier)
			}) => {},]
		);
	});
	// No message should reach the relaychain.
	Polkadot::execute_with(|| {
		assert_eq!(PolkadotSystem::events().len(), 0);
	})
}

#[test]
fn test_reserve_asset_transfer_from_regular_spiritnet_account_to_asset_hub() {
	MockNetwork::reset();

	let alice_account_id = get_account_id_from_seed::<sr25519::Public>(ALICE);
	let bob_account_id = get_account_id_from_seed::<sr25519::Public>(BOB);

	Spiritnet::execute_with(|| {
		assert_ok!(SpiritnetXcm::limited_reserve_transfer_assets(
			RawOrigin::Signed(alice_account_id.clone()).into(),
			Box::new(ParentThen(Junctions::X1(Junction::Parachain(asset_hub_polkadot::PARA_ID))).into()),
			Box::new(
				X1(Junction::AccountId32 {
					network: None,
					id: bob_account_id.into()
				})
				.into()
			),
			Box::new((Here, 1000 * ED).into()),
			0,
			WeightLimit::Unlimited,
		));

		type RuntimeEvent = <Spiritnet as Parachain>::RuntimeEvent;

		assert_expected_events!(
			Spiritnet,
			vec![RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Attempted {
				outcome: xcm::latest::Outcome::Complete(_)
			}) => {},
			]
		);
	});
	// No event on the relaychain (message is meant for asset hub
	Polkadot::execute_with(|| {
		assert_eq!(PolkadotSystem::events().len(), 0);
	});
	// Fails on AssetHub since spiritnet is not a trusted registrar
	AssetHub::execute_with(|| {
		type RuntimeEvent = <AssetHub as Parachain>::RuntimeEvent;

		assert_expected_events!(
			AssetHub,
			vec![RuntimeEvent::XcmpQueue(XcmpQueueEvent::Fail { .. }) => {},]
		);
	});
}

#[test]
fn test_reserve_asset_transfer_from_regular_asset_hub_account_to_spiritnet() {
	MockNetwork::reset();

	let alice_account_id = get_account_id_from_seed::<sr25519::Public>(ALICE);
	let bob_account_id = get_account_id_from_seed::<sr25519::Public>(BOB);

	AssetHub::execute_with(|| {
		assert_ok!(AssetHubXcm::limited_reserve_transfer_assets(
			RawOrigin::Signed(alice_account_id.clone()).into(),
			Box::new(ParentThen(Junctions::X1(Junction::Parachain(asset_hub_polkadot::PARA_ID))).into()),
			Box::new(
				X1(Junction::AccountId32 {
					network: None,
					id: bob_account_id.into()
				})
				.into()
			),
			Box::new((Here, 1000 * ED).into()),
			0,
			WeightLimit::Unlimited,
		));

		type RuntimeEvent = <AssetHub as Parachain>::RuntimeEvent;

		let bla = AssetHubSystem::events();

		println!("{:?}", bla);

		assert_expected_events!(
			AssetHub,
			vec![RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Attempted {
				outcome: xcm::latest::Outcome::Complete(_)
			}) => {},
			]
		);
	});
	// No event on the relaychain (message is meant for asset hub
	Polkadot::execute_with(|| {
		assert_eq!(PolkadotSystem::events().len(), 0);
	});
	// Fails on AssetHub since spiritnet is not a trusted registrar
	Spiritnet::execute_with(|| {
		type RuntimeEvent = <Spiritnet as Parachain>::RuntimeEvent;

		assert_expected_events!(
			Spiritnet,
			vec![RuntimeEvent::XcmpQueue(XcmpQueueEvent::Fail { .. }) => {},]
		);
	});
}

#[test]
fn test_teleport_asset_from_regular_spiritnet_account_to_asset_hub() {
	MockNetwork::reset();

	let alice_account_id = get_account_id_from_seed::<sr25519::Public>(ALICE);
	let bob_account_id = get_account_id_from_seed::<sr25519::Public>(BOB);

	Spiritnet::execute_with(|| {
		assert_err!(
			SpiritnetXcm::limited_teleport_assets(
				RawOrigin::Signed(alice_account_id.clone()).into(),
				Box::new(ParentThen(Junctions::X1(Junction::Parachain(asset_hub_polkadot::PARA_ID))).into()),
				Box::new(
					X1(Junction::AccountId32 {
						network: None,
						id: bob_account_id.into()
					})
					.into()
				),
				Box::new((Here, 1000 * ED).into()),
				0,
				WeightLimit::Unlimited,
			),
			DispatchError::Module(ModuleError {
				index: 83,
				error: [2, 0, 0, 0],
				message: Some("Filtered")
			})
		);
	});
	// No event on the relaychain (message is meant for asset hub)
	Polkadot::execute_with(|| {
		assert_eq!(PolkadotSystem::events().len(), 0);
	});
	// Fails on AsssetHub since spiritnet is not a trusted registrar.
	AssetHub::execute_with(|| {
		assert_eq!(AssetHubSystem::events().len(), 0);
	});
}

#[test]
fn test_sudo_call_from_relay_chain_to_spiritnet() {
	let code = vec![];

	let call: DoubleEncoded<()> = <Spiritnet as Parachain>::RuntimeCall::System(frame_system::Call::set_code { code })
		.encode()
		.into();
	let sudo_origin = <Polkadot as RelayChain>::RuntimeOrigin::root();
	let parachain_destination: VersionedMultiLocation = Polkadot::child_location_of(Spiritnet::para_id()).into();

	let weight_limit = WeightLimit::Unlimited;
	let require_weight_at_most = Weight::from_parts(1000000000, 200000);
	let origin_kind = OriginKind::Superuser;
	let check_origin = None;

	let xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution {
			weight_limit,
			check_origin,
		},
		Transact {
			origin_kind,
			require_weight_at_most,
			call,
		},
	]));

	//Send XCM message from relay chain
	Polkadot::execute_with(|| {
		assert_ok!(<Polkadot as PolkadotPallet>::XcmPallet::send(
			sudo_origin,
			Box::new(parachain_destination),
			Box::new(xcm)
		));

		type RuntimeEvent = <Polkadot as RelayChain>::RuntimeEvent;

		assert_expected_events!(
			Polkadot,
			vec![
				RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	Spiritnet::execute_with(|| {
		type SpiritnetRuntimeEvent = <Spiritnet as Parachain>::RuntimeEvent;

		assert_expected_events!(
			Spiritnet,
			vec![
				SpiritnetRuntimeEvent::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward {
					outcome: xcm::v3::Outcome::Error(xcm::v3::Error::Barrier),
					..
				}) => {},
			]
		);
	});

	// No event on the AssetHub message is meant for relay chain
	AssetHub::execute_with(|| {
		assert_eq!(AssetHubSystem::events().len(), 0);
	});
}

#[test]
fn test_sudo_call_from_asset_hub_to_spiritnet() {
	let code = vec![];

	let call: DoubleEncoded<()> = <Spiritnet as Parachain>::RuntimeCall::System(frame_system::Call::set_code { code })
		.encode()
		.into();
	let sudo_origin = <AssetHub as Parachain>::RuntimeOrigin::root();
	let parachain_destination: VersionedMultiLocation =
		ParentThen(Junctions::X1(Junction::Parachain(spiritnet::PARA_ID))).into();

	let weight_limit = WeightLimit::Unlimited;
	let require_weight_at_most = Weight::from_parts(1000000000, 200000);
	let origin_kind = OriginKind::Superuser;
	let check_origin = None;

	let xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution {
			weight_limit,
			check_origin,
		},
		Transact {
			origin_kind,
			require_weight_at_most,
			call,
		},
	]));

	//Send XCM message from AssetHub
	AssetHub::execute_with(|| {
		assert_ok!(<AssetHub as AssetHubPallet>::PolkadotXcm::send(
			sudo_origin,
			Box::new(parachain_destination),
			Box::new(xcm)
		));

		type RuntimeEvent = <AssetHub as Parachain>::RuntimeEvent;

		assert_expected_events!(
			AssetHub,
			vec![
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	Spiritnet::execute_with(|| {
		type SpiritnetRuntimeEvent = <Spiritnet as Parachain>::RuntimeEvent;
		assert_expected_events!(
			Spiritnet,
			vec![
				SpiritnetRuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Fail {  error: xcm::v3::Error::Barrier, ..  }) => {},
			]
		);
	});

	// No event on the relaychain (message is meant for asset hub)
	Polkadot::execute_with(|| {
		assert_eq!(PolkadotSystem::events().len(), 0);
	});
}

// TODO: Receive funds from assetHub
// TODO: create a DID from another chain
// TODO: use a DID (e.g. CType creation)
