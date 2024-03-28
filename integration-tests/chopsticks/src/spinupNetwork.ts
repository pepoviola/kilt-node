import { connectParachains, connectVertical } from '@acala-network/chopsticks'

import * as SpiritnetNetwork from './network/spiritnet.js'
import * as PolkadotNetwork from './network/polkadot.js'
import * as HydraDxNetwork from './network/hydraDx.js'
import { keysBob } from './utils.js'

/// Helper function to validate the storage of the chains. The chains will not produce any blocks on demand.
/// TODO: fix that
async function spinUpNetwork() {
	const spiritnetContext = await SpiritnetNetwork.getContext()
	const hydradxContext = await HydraDxNetwork.getContext()
	const polkadotContext = await PolkadotNetwork.getContext()

	await polkadotContext.dev.setStorage(PolkadotNetwork.defaultStorage(keysBob.address))
	await spiritnetContext.dev.setStorage(SpiritnetNetwork.defaultStorage(keysBob.address))
	await hydradxContext.dev.setStorage(HydraDxNetwork.defaultStorage(keysBob.address))

	// Setup network
	await connectVertical(polkadotContext.chain, spiritnetContext.chain)
	await connectVertical(polkadotContext.chain, hydradxContext.chain)
	await connectParachains([spiritnetContext.chain, hydradxContext.chain])
}

spinUpNetwork()
