import { beforeAll, afterAll } from 'vitest'
import { connectParachains, connectVertical } from '@acala-network/chopsticks'
import { setTimeout } from 'timers/promises'

import * as SpiritnetConfig from '../network/spiritnet.js'
import * as PolkadotConfig from '../network/polkadot.js'
import * as HydraDxConfig from '../network/hydraDx.js'
import type { Config } from '../network/types.js'
import { setStorage } from './utils.js'

export let spiritnetContext: Config
export let hydradxContext: Config
export let polkadotContext: Config

beforeAll(async () => {
	spiritnetContext = await SpiritnetConfig.getContext()
	hydradxContext = await HydraDxConfig.getContext()
	polkadotContext = await PolkadotConfig.getContext()

	// Setup network
	await connectVertical(polkadotContext.chain, spiritnetContext.chain)
	await connectVertical(polkadotContext.chain, hydradxContext.chain)
	await connectParachains([spiritnetContext.chain, hydradxContext.chain])

	const newBlockConfig = { count: 2 }
	// fixes api runtime disconnect warning
	await setTimeout(50)
	// Perform runtime upgrade and establish xcm connections.
	await Promise.all([
		polkadotContext.dev.newBlock(newBlockConfig),
		spiritnetContext.dev.newBlock(newBlockConfig),
		hydradxContext.dev.newBlock(newBlockConfig),
	])

	console.info('Runtime Upgrade completed')

	// set SafeXcmVersion to 3
	await setStorage(spiritnetContext, SpiritnetConfig.setSafeXcmVersion(3))

	// register Kilt in HydraDX
	await setStorage(hydradxContext, HydraDxConfig.registerKilt())
}, 300_000)

afterAll(async () => {
	// fixes api runtime disconnect warning
	await setTimeout(50)
	await Promise.all([spiritnetContext.teardown(), hydradxContext.teardown(), polkadotContext.teardown()])
})

export async function getFreeBalanceSpiritnet(account: string): Promise<bigint> {
	const accountInfo = await spiritnetContext.api.query.system.account(account)
	return accountInfo.data.free.toBigInt()
}

export async function getFreeBalanceHydraDxKilt(account: string): Promise<bigint> {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const accountInfo: any = await hydradxContext.api.query.tokens.accounts(account, HydraDxConfig.kiltTokenId)
	return accountInfo.free.toBigInt()
}
