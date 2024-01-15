// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2023 BOTLabs GmbH

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

//! Module to deal with cross-chain Merkle proof as generated by the KILT chain.

use did::{
	did_details::{DidPublicKey, DidPublicKeyDetails},
	DidSignature, DidVerificationKeyRelationship,
};
use frame_support::{ensure, RuntimeDebug};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::ConstU32;
use sp_runtime::{
	traits::{Hash, Header},
	BoundedVec, SaturatedConversion,
};
use sp_std::vec::Vec;
use sp_trie::{verify_trie_proof, LayoutV1};

use crate::{
	common::{calculate_dip_identity_commitment_storage_key_for_runtime, calculate_parachain_head_storage_key},
	state_proofs::{verify_storage_value_proof, MerkleProofError},
	traits::GetWithArg,
	utils::OutputOf,
};

pub struct ProviderHeadProof<RelayBlockNumber, const MAX_LEAVE_COUNT: u32, const MAX_LEAVE_SIZE: u32> {
	pub(crate) relay_block_number: RelayBlockNumber,
	pub(crate) proof: BoundedVec<BoundedVec<u8, ConstU32<MAX_LEAVE_SIZE>>, ConstU32<MAX_LEAVE_COUNT>>,
}

pub struct DipCommitmentProof<const MAX_LEAVE_COUNT: u32, const MAX_LEAVE_SIZE: u32>(
	pub(crate) BoundedVec<BoundedVec<u8, ConstU32<MAX_LEAVE_SIZE>>, ConstU32<MAX_LEAVE_COUNT>>,
);

pub struct DidMerkleProof<
	ProviderDidKeyId,
	ProviderAccountId,
	ProviderBlockNumber,
	ProviderWeb3Name,
	ProviderLinkableAccountId,
	const MAX_BLINDED_LEAVE_COUNT: u32,
	const MAX_BLINDED_LEAVE_SIZE: u32,
	const MAX_LEAVES_REVEALED: u32,
> {
	pub(crate) blinded: BoundedVec<BoundedVec<u8, ConstU32<MAX_BLINDED_LEAVE_SIZE>>, ConstU32<MAX_BLINDED_LEAVE_COUNT>>,
	pub(crate) revealed: BoundedVec<
		RevealedDidMerkleProofLeaf<
			ProviderDidKeyId,
			ProviderAccountId,
			ProviderBlockNumber,
			ProviderWeb3Name,
			ProviderLinkableAccountId,
		>,
		ConstU32<MAX_LEAVES_REVEALED>,
	>,
}

/// A DID signature anchored to a specific block height.
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, TypeInfo)]
pub struct TimeBoundDidSignature<BlockNumber> {
	/// The signature.
	pub(crate) signature: DidSignature,
	/// The block number until the signature is to be considered valid.
	pub(crate) valid_until: BlockNumber,
}

pub struct DipDidProof<
	RelayBlockNumber,
	KiltDidKeyId,
	KiltAccountId,
	KiltBlockNumber,
	KiltWeb3Name,
	KiltLinkableAccountId,
	ConsumerBlockNumber,
	const MAX_PROVIDER_HEAD_PROOF_LEAVE_COUNT: u32 = 10,
	const MAX_PROVIDER_HEAD_PROOF_LEAVE_SIZE: u32 = 128,
	const MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT: u32 = 10,
	const MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE: u32 = 128,
	const MAX_DID_MERKLE_PROOF_LEAVE_COUNT: u32 = 10,
	const MAX_DID_MERKLE_PROOF_LEAVE_SIZE: u32 = 128,
	const MAX_DID_MERKLE_LEAVES_REVEALED: u32 = 10,
> {
	pub(crate) provider_head_proof:
		ProviderHeadProof<RelayBlockNumber, MAX_PROVIDER_HEAD_PROOF_LEAVE_COUNT, MAX_PROVIDER_HEAD_PROOF_LEAVE_SIZE>,
	pub(crate) dip_commitment_proof:
		DipCommitmentProof<MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT, MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE>,
	pub(crate) dip_proof: DidMerkleProof<
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
		MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	>,
	pub(crate) signature: TimeBoundDidSignature<ConsumerBlockNumber>,
}

pub enum Error {
	ProviderHeadProof(MerkleProofError),
	RelayStateRootNotFound,
}

impl<
		RelayBlockNumber,
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		const MAX_PROVIDER_HEAD_PROOF_LEAVE_COUNT: u32,
		const MAX_PROVIDER_HEAD_PROOF_LEAVE_SIZE: u32,
		const MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT: u32,
		const MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE: u32,
		const MAX_DID_MERKLE_PROOF_LEAVE_COUNT: u32,
		const MAX_DID_MERKLE_PROOF_LEAVE_SIZE: u32,
		const MAX_DID_MERKLE_LEAVES_REVEALED: u32,
	>
	DipDidProof<
		RelayBlockNumber,
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		MAX_PROVIDER_HEAD_PROOF_LEAVE_COUNT,
		MAX_PROVIDER_HEAD_PROOF_LEAVE_SIZE,
		MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT,
		MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
		MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	>
{
	pub fn verify_top_level_head_proof_for_provider_and_state_root<RelayHasher, ProviderHeader>(
		self,
		provider_para_id: u32,
		relay_state_root: &OutputOf<RelayHasher>,
	) -> Result<
		RelayVerifiedDipProof<
			OutputOf<ProviderHeader::Hashing>,
			KiltDidKeyId,
			KiltAccountId,
			KiltBlockNumber,
			KiltWeb3Name,
			KiltLinkableAccountId,
			ConsumerBlockNumber,
			MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT,
			MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE,
			MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
			MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
			MAX_DID_MERKLE_LEAVES_REVEALED,
		>,
		Error,
	>
	where
		RelayHasher: Hash,
		ProviderHeader: Decode + Header,
	{
		let provider_head_storage_key = calculate_parachain_head_storage_key(provider_para_id);
		let provider_header = verify_storage_value_proof::<_, RelayHasher, ProviderHeader>(
			&provider_head_storage_key,
			*relay_state_root,
			self.provider_head_proof.proof.into_iter().map(|i| i.into()),
		)
		.map_err(Error::ProviderHeadProof)?;
		Ok(RelayVerifiedDipProof {
			state_root: *provider_header.state_root(),
			dip_commitment_proof: self.dip_commitment_proof,
			dip_proof: self.dip_proof,
			signature: self.signature,
		})
	}

	pub fn verify_top_level_head_proof_for_provider<RelayHasher, StateRootStore, ProviderHeader>(
		self,
		provider_para_id: u32,
	) -> Result<
		RelayVerifiedDipProof<
			OutputOf<ProviderHeader::Hashing>,
			KiltDidKeyId,
			KiltAccountId,
			KiltBlockNumber,
			KiltWeb3Name,
			KiltLinkableAccountId,
			ConsumerBlockNumber,
			MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT,
			MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE,
			MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
			MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
			MAX_DID_MERKLE_LEAVES_REVEALED,
		>,
		Error,
	>
	where
		RelayHasher: Hash,
		StateRootStore: GetWithArg<RelayBlockNumber, Result = Option<OutputOf<RelayHasher>>>,
		ProviderHeader: Decode + Header,
	{
		let relay_state_root =
			StateRootStore::get(&self.provider_head_proof.relay_block_number).ok_or(Error::RelayStateRootNotFound)?;
		self.verify_top_level_head_proof_for_provider_and_state_root::<RelayHasher, ProviderHeader>(
			provider_para_id,
			&relay_state_root,
		)
	}
}

pub struct RelayVerifiedDipProof<
	StateRoot,
	KiltDidKeyId,
	KiltAccountId,
	KiltBlockNumber,
	KiltWeb3Name,
	KiltLinkableAccountId,
	ConsumerBlockNumber,
	const MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT: u32 = 10,
	const MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE: u32 = 128,
	const MAX_DID_MERKLE_PROOF_LEAVE_COUNT: u32 = 10,
	const MAX_DID_MERKLE_PROOF_LEAVE_SIZE: u32 = 128,
	const MAX_DID_MERKLE_LEAVES_REVEALED: u32 = 10,
> {
	pub(crate) state_root: StateRoot,
	pub(crate) dip_commitment_proof:
		DipCommitmentProof<MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT, MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE>,
	pub(crate) dip_proof: DidMerkleProof<
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
		MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	>,
	pub(crate) signature: TimeBoundDidSignature<ConsumerBlockNumber>,
}

impl<
		StateRoot,
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		const MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT: u32,
		const MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE: u32,
		const MAX_DID_MERKLE_PROOF_LEAVE_COUNT: u32,
		const MAX_DID_MERKLE_PROOF_LEAVE_SIZE: u32,
		const MAX_DID_MERKLE_LEAVES_REVEALED: u32,
	>
	RelayVerifiedDipProof<
		StateRoot,
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		MAX_DIP_COMMITMENT_PROOF_LEAVE_COUNT,
		MAX_DIP_COMMITMENT_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
		MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	>
{
	pub fn verify_dip_commitment_proof_for_subject<MerkleHasher, ProviderRuntime, Commitment>(
		self,
		subject: &ProviderRuntime::Identifier,
	) -> Result<
		CommitmentVerifiedProof<
			Commitment,
			KiltDidKeyId,
			KiltAccountId,
			KiltBlockNumber,
			KiltWeb3Name,
			KiltLinkableAccountId,
			ConsumerBlockNumber,
			MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
			MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
			MAX_DID_MERKLE_LEAVES_REVEALED,
		>,
		Error,
	>
	where
		MerkleHasher: Hash,
		StateRoot: Into<OutputOf<MerkleHasher>>,
		ProviderRuntime: pallet_dip_provider::Config,
		Commitment: Decode,
		OutputOf<MerkleHasher>: Into<Commitment>,
	{
		let dip_commitment_storage_key =
			calculate_dip_identity_commitment_storage_key_for_runtime::<ProviderRuntime>(subject, 0);
		let dip_commitment = verify_storage_value_proof::<_, MerkleHasher, Commitment>(
			&dip_commitment_storage_key,
			self.state_root.into(),
			self.dip_commitment_proof.0.into_iter().map(|i| i.into()),
		)
		.map_err(Error::ProviderHeadProof)?;
		Ok(CommitmentVerifiedProof {
			dip_commitment,
			dip_proof: self.dip_proof,
			signature: self.signature,
		})
	}
}

pub(crate) struct CommitmentVerifiedProof<
	Commitment,
	KiltDidKeyId,
	KiltAccountId,
	KiltBlockNumber,
	KiltWeb3Name,
	KiltLinkableAccountId,
	ConsumerBlockNumber,
	const MAX_DID_MERKLE_PROOF_LEAVE_COUNT: u32 = 10,
	const MAX_DID_MERKLE_PROOF_LEAVE_SIZE: u32 = 128,
	const MAX_DID_MERKLE_LEAVES_REVEALED: u32 = 10,
> {
	pub(crate) dip_commitment: Commitment,
	pub(crate) dip_proof: DidMerkleProof<
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
		MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	>,
	pub(crate) signature: TimeBoundDidSignature<ConsumerBlockNumber>,
}

impl<
		Commitment,
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		const MAX_DID_MERKLE_PROOF_LEAVE_COUNT: u32,
		const MAX_DID_MERKLE_PROOF_LEAVE_SIZE: u32,
		const MAX_DID_MERKLE_LEAVES_REVEALED: u32,
	>
	CommitmentVerifiedProof<
		Commitment,
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		MAX_DID_MERKLE_PROOF_LEAVE_COUNT,
		MAX_DID_MERKLE_PROOF_LEAVE_SIZE,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	> where
	KiltDidKeyId: Encode,
	KiltAccountId: Encode,
	KiltBlockNumber: Encode,
	KiltWeb3Name: Encode,
	KiltLinkableAccountId: Encode,
{
	pub fn verify_dip_proof<MerkleHasher>(
		self,
	) -> Result<
		DipVerifiedProof<
			KiltDidKeyId,
			KiltAccountId,
			KiltBlockNumber,
			KiltWeb3Name,
			KiltLinkableAccountId,
			ConsumerBlockNumber,
			MAX_DID_MERKLE_LEAVES_REVEALED,
		>,
		Error,
	>
	where
		MerkleHasher: Hash<Output = Commitment>,
	{
		let mut revealed_keys = self
			.dip_proof
			.revealed
			.iter()
			.take(MAX_DID_MERKLE_LEAVES_REVEALED.saturated_into());

		// If there are more keys than MAX_LEAVES_REVEALED, bail out.
		ensure!(
			revealed_keys.next().is_none(),
			// TODO: Change
			Error::RelayStateRootNotFound,
		);

		let proof_leaves_key_value_pairs: Vec<(Vec<u8>, Option<Vec<u8>>)> = revealed_keys
			.by_ref()
			.map(|revealed_leaf| (revealed_leaf.encoded_key(), Some(revealed_leaf.encoded_value())))
			.collect();

		verify_trie_proof::<LayoutV1<MerkleHasher>, _, _, _>(
			&self.dip_commitment,
			self.dip_proof
				.blinded
				.into_iter()
				.map(|l| l.into_inner())
				.collect::<Vec<_>>()
				.as_slice(),
			&proof_leaves_key_value_pairs,
		)
		// TODO: Change
		.map_err(|_| Error::RelayStateRootNotFound)?;

		Ok(DipVerifiedProof {
			revealed_leaves: self.dip_proof.revealed,
			signature: self.signature,
		})
	}
}

pub(crate) struct DipVerifiedProof<
	KiltDidKeyId,
	KiltAccountId,
	KiltBlockNumber,
	KiltWeb3Name,
	KiltLinkableAccountId,
	ConsumerBlockNumber,
	const MAX_DID_MERKLE_LEAVES_REVEALED: u32 = 10,
> {
	pub(crate) revealed_leaves: BoundedVec<
		RevealedDidMerkleProofLeaf<KiltDidKeyId, KiltAccountId, KiltBlockNumber, KiltWeb3Name, KiltLinkableAccountId>,
		ConstU32<MAX_DID_MERKLE_LEAVES_REVEALED>,
	>,
	pub(crate) signature: TimeBoundDidSignature<ConsumerBlockNumber>,
}

impl<
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		const MAX_DID_MERKLE_LEAVES_REVEALED: u32,
	>
	DipVerifiedProof<
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		ConsumerBlockNumber,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	> where
	ConsumerBlockNumber: PartialOrd,
{
	pub fn verify_signature_time(
		self,
		block_number: &ConsumerBlockNumber,
	) -> Result<
		DipSignatureTimeVerifiedProof<
			KiltDidKeyId,
			KiltAccountId,
			KiltBlockNumber,
			KiltWeb3Name,
			KiltLinkableAccountId,
			MAX_DID_MERKLE_LEAVES_REVEALED,
		>,
		Error,
	> {
		ensure!(
			self.signature.valid_until >= *block_number,
			Error::RelayStateRootNotFound
		);
		Ok(DipSignatureTimeVerifiedProof {
			revealed_leaves: self.revealed_leaves,
			signature: self.signature.signature,
		})
	}
}

pub(crate) struct DipSignatureTimeVerifiedProof<
	KiltDidKeyId,
	KiltAccountId,
	KiltBlockNumber,
	KiltWeb3Name,
	KiltLinkableAccountId,
	const MAX_DID_MERKLE_LEAVES_REVEALED: u32 = 10,
> {
	pub(crate) revealed_leaves: BoundedVec<
		RevealedDidMerkleProofLeaf<KiltDidKeyId, KiltAccountId, KiltBlockNumber, KiltWeb3Name, KiltLinkableAccountId>,
		ConstU32<MAX_DID_MERKLE_LEAVES_REVEALED>,
	>,
	pub(crate) signature: DidSignature,
}

impl<
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		const MAX_DID_MERKLE_LEAVES_REVEALED: u32,
	>
	DipSignatureTimeVerifiedProof<
		KiltDidKeyId,
		KiltAccountId,
		KiltBlockNumber,
		KiltWeb3Name,
		KiltLinkableAccountId,
		MAX_DID_MERKLE_LEAVES_REVEALED,
	>
{
	pub fn retrieve_signing_leaf_for_payload<'a>(
		self,
		payload: &[u8],
	) -> Result<
		DipSignatureVerifiedProof<
			'a,
			KiltDidKeyId,
			KiltAccountId,
			KiltBlockNumber,
			KiltWeb3Name,
			KiltLinkableAccountId,
			MAX_DID_MERKLE_LEAVES_REVEALED,
		>,
		Error,
	> {
		let revealed_verification_keys = self.revealed_leaves.iter().filter(|leaf| {
			matches!(
				leaf,
				RevealedDidMerkleProofLeaf::DidKey(RevealedDidKey {
					relationship: DidKeyRelationship::Verification(verification_relationship),
					..
				})
			)
		});
		let signing_key = revealed_verification_keys
			.find(|revealed_verification_key| {
				let RevealedDidMerkleProofLeaf::DidKey(RevealedDidKey {
					details:
						DidPublicKeyDetails {
							key: DidPublicKey::PublicVerificationKey(verification_key),
							..
						},
					..
				}) = revealed_verification_key
				else {
					return false;
				};
				verification_key.verify_signature(payload, &self.signature).is_ok()
				// TODO: Change
			})
			.ok_or(Error::RelayStateRootNotFound)?;
		Ok(DipSignatureVerifiedProof {
			revealed_leaves: self.revealed_leaves,
			// TODO: Fix this compilation issue, and then we are golden!
			signing_leaf: RevealedDidMerkleProofLeaf::DidKey(signing_key),
		})
	}
}

pub(crate) struct DipSignatureVerifiedProof<
	'a,
	KiltDidKeyId,
	KiltAccountId,
	KiltBlockNumber,
	KiltWeb3Name,
	KiltLinkableAccountId,
	const MAX_DID_MERKLE_LEAVES_REVEALED: u32 = 10,
> {
	pub revealed_leaves: BoundedVec<
		RevealedDidMerkleProofLeaf<KiltDidKeyId, KiltAccountId, KiltBlockNumber, KiltWeb3Name, KiltLinkableAccountId>,
		ConstU32<MAX_DID_MERKLE_LEAVES_REVEALED>,
	>,
	pub signing_leaf: &'a RevealedDidKey<KiltDidKeyId, KiltAccountId, KiltBlockNumber>,
}

/// Relationship of a key to a DID Document.
#[derive(Clone, Copy, RuntimeDebug, Encode, Decode, PartialEq, Eq, TypeInfo, PartialOrd, Ord, MaxEncodedLen)]
pub enum DidKeyRelationship {
	Encryption,
	Verification(DidVerificationKeyRelationship),
}

impl From<DidVerificationKeyRelationship> for DidKeyRelationship {
	fn from(value: DidVerificationKeyRelationship) -> Self {
		Self::Verification(value)
	}
}

impl TryFrom<DidKeyRelationship> for DidVerificationKeyRelationship {
	type Error = ();

	fn try_from(value: DidKeyRelationship) -> Result<Self, Self::Error> {
		if let DidKeyRelationship::Verification(rel) = value {
			Ok(rel)
		} else {
			Err(())
		}
	}
}

/// All possible Merkle leaf types that can be revealed as part of a DIP
/// identity Merkle proof.
#[derive(Clone, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum RevealedDidMerkleProofLeaf<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId> {
	DidKey(RevealedDidKey<KeyId, BlockNumber, AccountId>),
	Web3Name(RevealedWeb3Name<Web3Name, BlockNumber>),
	LinkedAccount(RevealedAccountId<LinkedAccountId>),
}

impl<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId> From<RevealedDidKey<KeyId, BlockNumber, AccountId>>
	for RevealedDidMerkleProofLeaf<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
{
	fn from(value: RevealedDidKey<KeyId, BlockNumber, AccountId>) -> Self {
		Self::DidKey(value)
	}
}

impl<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId> From<RevealedWeb3Name<Web3Name, BlockNumber>>
	for RevealedDidMerkleProofLeaf<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
{
	fn from(value: RevealedWeb3Name<Web3Name, BlockNumber>) -> Self {
		Self::Web3Name(value)
	}
}

impl<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId> From<RevealedAccountId<LinkedAccountId>>
	for RevealedDidMerkleProofLeaf<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
{
	fn from(value: RevealedAccountId<LinkedAccountId>) -> Self {
		Self::LinkedAccount(value)
	}
}

#[cfg(feature = "runtime-benchmarks")]
impl<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId> Default
	for RevealedDidMerkleProofLeaf<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
where
	KeyId: Default,
	BlockNumber: Default,
{
	fn default() -> Self {
		RevealedDidKey {
			id: KeyId::default(),
			relationship: DidVerificationKeyRelationship::Authentication.into(),
			details: DidPublicKeyDetails {
				key: did::did_details::DidVerificationKey::Ed25519(sp_core::ed25519::Public::from_raw([0u8; 32]))
					.into(),
				block_number: BlockNumber::default(),
			},
		}
		.into()
	}
}

impl<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
	RevealedDidMerkleProofLeaf<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
where
	KeyId: Encode,
	Web3Name: Encode,
	LinkedAccountId: Encode,
{
	pub fn encoded_key(&self) -> Vec<u8> {
		match self {
			RevealedDidMerkleProofLeaf::DidKey(RevealedDidKey { id, relationship, .. }) => (id, relationship).encode(),
			RevealedDidMerkleProofLeaf::Web3Name(RevealedWeb3Name { web3_name, .. }) => web3_name.encode(),
			RevealedDidMerkleProofLeaf::LinkedAccount(RevealedAccountId(account_id)) => account_id.encode(),
		}
	}
}

impl<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
	RevealedDidMerkleProofLeaf<KeyId, AccountId, BlockNumber, Web3Name, LinkedAccountId>
where
	AccountId: Encode,
	BlockNumber: Encode,
{
	pub fn encoded_value(&self) -> Vec<u8> {
		match self {
			RevealedDidMerkleProofLeaf::DidKey(RevealedDidKey { details, .. }) => details.encode(),
			RevealedDidMerkleProofLeaf::Web3Name(RevealedWeb3Name { claimed_at, .. }) => claimed_at.encode(),
			RevealedDidMerkleProofLeaf::LinkedAccount(_) => ().encode(),
		}
	}
}

/// The details of a DID key after it has been successfully verified in a Merkle
/// proof.
#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Eq, PartialOrd, Ord, RuntimeDebug, TypeInfo)]
pub struct RevealedDidKey<KeyId, BlockNumber, AccountId> {
	/// The key ID, according to the provider's definition.
	pub id: KeyId,
	/// The key relationship to the subject's DID Document.
	pub relationship: DidKeyRelationship,
	/// The details of the DID Key, including its creation block number on the
	/// provider chain.
	pub details: DidPublicKeyDetails<BlockNumber, AccountId>,
}

/// The details of a web3name after it has been successfully verified in a
/// Merkle proof.
#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Eq, PartialOrd, Ord, RuntimeDebug, TypeInfo)]
pub struct RevealedWeb3Name<Web3Name, BlockNumber> {
	/// The web3name.
	pub web3_name: Web3Name,
	/// The block number on the provider chain in which it was linked to the DID
	/// subject.
	pub claimed_at: BlockNumber,
}

/// The details of an account after it has been successfully verified in a
/// Merkle proof.
#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Eq, PartialOrd, Ord, RuntimeDebug, TypeInfo)]
pub struct RevealedAccountId<AccountId>(pub AccountId);
