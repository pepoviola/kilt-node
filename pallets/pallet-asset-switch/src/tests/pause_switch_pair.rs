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

use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use sp_runtime::DispatchError;

use crate::{
	mock::{
		ExtBuilder, MockRuntime, NewSwitchPairInfo, System, ASSET_HUB_LOCATION, REMOTE_ERC20_ASSET_ID, XCM_ASSET_FEE,
	},
	switch::SwitchPairStatus,
	Error, Event, Pallet, SwitchPair,
};

#[test]
fn successful() {
	// Stopping a running switch pair generates an event.
	ExtBuilder::default()
		.with_switch_pair_info(NewSwitchPairInfo {
			circulating_supply: 0,
			min_remote_balance: 0,
			pool_account: [0u8; 32].into(),
			remote_asset_id: REMOTE_ERC20_ASSET_ID.into(),
			remote_fee: XCM_ASSET_FEE.into(),
			remote_reserve_location: ASSET_HUB_LOCATION.into(),
			status: SwitchPairStatus::Running,
			total_issuance: 1_000,
		})
		.build()
		.execute_with(|| {
			assert_ok!(Pallet::<MockRuntime>::pause_switch_pair(RawOrigin::Root.into()));
			assert_eq!(
				SwitchPair::<MockRuntime>::get().unwrap().status,
				SwitchPairStatus::Paused
			);
			assert!(System::events().into_iter().map(|e| e.event).any(|e| e
				== Event::<MockRuntime>::SwitchPairPaused {
					remote_asset_id: REMOTE_ERC20_ASSET_ID.into()
				}
				.into()));
		});
	// Stopping a non-running switch pair generates no event.
	ExtBuilder::default()
		.with_switch_pair_info(NewSwitchPairInfo {
			circulating_supply: 0,
			min_remote_balance: 0,
			pool_account: [0u8; 32].into(),
			remote_asset_id: REMOTE_ERC20_ASSET_ID.into(),
			remote_fee: XCM_ASSET_FEE.into(),
			remote_reserve_location: ASSET_HUB_LOCATION.into(),
			status: SwitchPairStatus::Paused,
			total_issuance: 1_000,
		})
		.build()
		.execute_with(|| {
			assert_ok!(Pallet::<MockRuntime>::pause_switch_pair(RawOrigin::Root.into()));
			assert_eq!(
				SwitchPair::<MockRuntime>::get().unwrap().status,
				SwitchPairStatus::Paused
			);
			assert!(System::events().into_iter().map(|e| e.event).all(|e| e
				!= Event::<MockRuntime>::SwitchPairPaused {
					remote_asset_id: REMOTE_ERC20_ASSET_ID.into()
				}
				.into()));
		});
}

#[test]
fn fails_on_non_existing_pair() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Pallet::<MockRuntime>::pause_switch_pair(RawOrigin::Root.into()),
			Error::<MockRuntime>::SwitchPairNotFound
		);
	});
}

#[test]
fn fails_on_invalid_origin() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Pallet::<MockRuntime>::pause_switch_pair(RawOrigin::None.into()),
			DispatchError::BadOrigin
		);
	});
}
