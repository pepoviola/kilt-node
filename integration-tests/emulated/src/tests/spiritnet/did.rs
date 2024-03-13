use crate::{
	mock::{
		network::MockNetworkPolkadot,
		para_chains::{spiritnet, AssetHubPolkadot, AssetHubPolkadotPallet, Spiritnet},
		relay_chains::Polkadot,
	},
	utils::UNIT,
};
use did::did_details::DidVerificationKey;
use frame_support::traits::fungible::hold::Inspect;
use frame_support::{assert_ok, traits::fungible::Mutate};
use parity_scale_codec::Encode;
use runtime_common::AccountId;
use xcm::{v3::WeightLimit, DoubleEncoded, VersionedMultiLocation, VersionedXcm};
use xcm_emulator::{
	assert_expected_events, Here,
	Instruction::{BuyExecution, Transact, WithdrawAsset},
	Junction, Junctions, OriginKind, Parachain, ParentThen, TestExt, Weight, Xcm,
};

#[test]
fn test_did_creation_from_asset_hub() {
	MockNetworkPolkadot::reset();

	// create the sovereign account of AssetHub
	let asset_hub_sovereign_account =
		Spiritnet::sovereign_account_id_of(Spiritnet::sibling_location_of(AssetHubPolkadot::para_id()));

	let call: DoubleEncoded<()> = <Spiritnet as Parachain>::RuntimeCall::Did(did::Call::create_from_account {
		authentication_key: DidVerificationKey::Account(asset_hub_sovereign_account.clone()),
	})
	.encode()
	.into();

	let sudo_origin = <AssetHubPolkadot as Parachain>::RuntimeOrigin::root();

	let parachain_destination: VersionedMultiLocation =
		ParentThen(Junctions::X1(Junction::Parachain(spiritnet::PARA_ID))).into();

	// the Weight parts are copied from logs.
	let require_weight_at_most = Weight::from_parts(10_000_600_000_000, 200_000_000_000);
	let origin_kind = OriginKind::SovereignAccount;

	let init_balance = UNIT * 10;
	let withdraw_balance = init_balance / 2;

	let xcm = VersionedXcm::from(Xcm(vec![
		WithdrawAsset((Here, withdraw_balance).into()),
		BuyExecution {
			fees: (Here, withdraw_balance).into(),
			weight_limit: WeightLimit::Unlimited,
		},
		Transact {
			origin_kind,
			require_weight_at_most,
			call,
		},
	]));

	// give the sovereign account of AssetHub some coins.
	Spiritnet::execute_with(|| {
		<spiritnet_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
	});

	//Send XCM message from AssetHub
	AssetHubPolkadot::execute_with(|| {
		assert_ok!(<AssetHubPolkadot as AssetHubPolkadotPallet>::PolkadotXcm::send(
			sudo_origin,
			Box::new(parachain_destination),
			Box::new(xcm)
		));

		type RuntimeEvent = <AssetHubPolkadot as Parachain>::RuntimeEvent;
		assert_expected_events!(
			AssetHubPolkadot,
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
				SpiritnetRuntimeEvent::Did(did::Event::DidCreated(account, did_identifier)) => {
					account: account == &asset_hub_sovereign_account,
					did_identifier:  did_identifier == &asset_hub_sovereign_account,
				},
				SpiritnetRuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. }) => {},
			]
		);

		// we also expect that the sovereignAccount of AssetHub has some coins now
		let balance_on_hold = <<Spiritnet as Parachain>::Balances as Inspect<AccountId>>::balance_on_hold(
			&spiritnet_runtime::RuntimeHoldReason::from(did::HoldReason::Deposit),
			&asset_hub_sovereign_account,
		);

		// since a did is created, 2 of the free balance should now be on hold
		assert_eq!(balance_on_hold, UNIT * 2);
	});

	// No event on the relaychain (message is meant for Spiritnet)
	Polkadot::execute_with(|| {
		assert_eq!(Polkadot::events().len(), 0);
	});
}
