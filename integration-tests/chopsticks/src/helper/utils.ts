import { Keyring } from '@polkadot/keyring'
import { u8aToHex } from '@polkadot/util'
import { decodeAddress } from '@polkadot/util-crypto'
import { ExpectStatic } from 'vitest'

const keyring = new Keyring({ type: 'ed25519', ss58Format: 38 })

export const keysAlice = keyring.addFromUri('//alice', undefined, 'ed25519')
export const keysBob = keyring.addFromUri('//bob', undefined, 'ed25519')
export const keysCharlie = keyring.addFromUri('//charlie', undefined, 'ed25519')

/**
 * Converts a given value to a Number
 */
export function toNumber(value: string | undefined): number | undefined {
	if (value === undefined) {
		return undefined
	}

	return Number(value)
}

/**
 * Converts a given address to its hexadecimal representation.
 *
 * @param addr - The address to be converted. It should be a string representation of an address.
 */
export function hexAddress(addr: string) {
	return u8aToHex(decodeAddress(addr))
}

/**
 * Validates if the received balance is within the expected range considering a certain precision.
 *
 * @param previousBalance - The balance before the operation.
 * @param receivedBalance - The balance after the operation.
 * @param deltaBalance - The expected change in balance.
 * @param expect - The assertion function from the testing library.
 * @param precision - The precision of the balance validation. It must be a value between 0 and 100.
 *
 * The function calculates the expected balance by adding the deltaBalance to the previousBalance.
 * It then calculates the lower and upper bounds of the expected balance considering the precision.
 * The received balance is then checked if it falls within the range of the lower and upper bounds.
 *
 * If the precision is not between 0 and 100, the function throws an error.
 *
 * @throws {Error} If the precision is not between 0 and 100.
 */
export function validateBalanceWithPrecision(
	previousBalance: bigint,
	receivedBalance: bigint,
	deltaBalance: bigint,
	expect: ExpectStatic,
	precision: bigint
) {
	if (precision < BigInt(0) || precision > BigInt(100)) {
		throw new Error('Precision must be between 0 and 100')
	}

	const allowedError = BigInt(100) - precision
	const expectedBalance = previousBalance + deltaBalance

	const lowerBound = expectedBalance - (expectedBalance * allowedError) / BigInt(100)
	const upperBound = expectedBalance + (expectedBalance * allowedError) / BigInt(100)

	expect(receivedBalance).toBeGreaterThanOrEqual(lowerBound)
	expect(receivedBalance).toBeLessThanOrEqual(upperBound)
}

export const KILT = BigInt(1e15)
export const DOT = BigInt(1e10)
export const HDX = BigInt(1e12)

export const initialBalanceKILT = BigInt(100) * KILT
export const initialBalanceDOT = BigInt(100) * DOT
export const initialBalanceHDX = BigInt(100) * HDX
