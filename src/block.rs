#[cfg(test)]
#[path = "block_test.rs"]
mod block_test;

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::core::{ContractAddress, GlobalRoot, SequencerPublicKey};
use crate::crypto::{verify_message_hash_signature, CryptoError, Signature};
use crate::hash::{poseidon_hash_array, StarkHash};
use crate::serde_utils::{BytesAsHex, PrefixedBytesAsHex};
use crate::transaction::{Transaction, TransactionHash, TransactionOutput};

/// A block.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody,
}

/// The header of a [Block](`crate::block::Block`).
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct BlockHeader {
    // TODO: Consider removing the block hash from the header (note it can be computed from
    // the rest of the fields.
    pub block_hash: BlockHash,
    pub parent_hash: BlockHash,
    pub block_number: BlockNumber,
    pub eth_l1_gas_price: GasPrice,
    pub strk_l1_gas_price: GasPrice,
    pub state_root: GlobalRoot,
    pub sequencer: ContractAddress,
    pub timestamp: BlockTimestamp,
    // TODO: add missing commitments.
}

/// The [transactions](`crate::transaction::Transaction`) and their
/// [outputs](`crate::transaction::TransactionOutput`) in a [block](`crate::block::Block`).
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockBody {
    pub transactions: Vec<Transaction>,
    pub transaction_outputs: Vec<TransactionOutput>,
    pub transaction_hashes: Vec<TransactionHash>,
}

/// The status of a [Block](`crate::block::Block`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub enum BlockStatus {
    /// A pending block; i.e., a block that is yet to be closed.
    #[serde(rename = "PENDING")]
    Pending,
    /// A block that was created on L2.
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    /// A block that was accepted on L1.
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
    /// A block rejected on L1.
    #[serde(rename = "REJECTED")]
    Rejected,
}

/// The hash of a [Block](`crate::block::Block`).
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Deserialize,
    Serialize,
    PartialOrd,
    Ord,
    Display,
)]
pub struct BlockHash(pub StarkHash);

/// The number of a [Block](`crate::block::Block`).
#[derive(
    Debug,
    Default,
    Copy,
    Display,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Deserialize,
    Serialize,
    PartialOrd,
    Ord,
)]
pub struct BlockNumber(pub u64);

impl BlockNumber {
    pub fn next(&self) -> BlockNumber {
        BlockNumber(self.0 + 1)
    }

    pub fn prev(&self) -> Option<BlockNumber> {
        match self.0 {
            0 => None,
            i => Some(BlockNumber(i - 1)),
        }
    }

    pub fn iter_up_to(&self, up_to: Self) -> impl Iterator<Item = BlockNumber> {
        let range = self.0..up_to.0;
        range.map(Self)
    }
}

/// The gas price at a [Block](`crate::block::Block`).
#[derive(
    Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
#[serde(from = "PrefixedBytesAsHex<16_usize>", into = "PrefixedBytesAsHex<16_usize>")]
pub struct GasPrice(pub u128);

impl From<PrefixedBytesAsHex<16_usize>> for GasPrice {
    fn from(val: PrefixedBytesAsHex<16_usize>) -> Self {
        GasPrice(u128::from_be_bytes(val.0))
    }
}

impl From<GasPrice> for PrefixedBytesAsHex<16_usize> {
    fn from(val: GasPrice) -> Self {
        BytesAsHex(val.0.to_be_bytes())
    }
}

/// The timestamp of a [Block](`crate::block::Block`).
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct BlockTimestamp(pub u64);

/// The signature of a [Block](`crate::block::Block`), signed by the sequencer. The signed message
/// is defined as poseidon_hash(block_hash, state_diff_commitment).
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct BlockSignature(pub Signature);

/// The error type returned from the block verification functions.
#[derive(thiserror::Error, Clone, Debug)]
pub enum BlockVerificationError {
    #[error("Failed to verify the signature of block {block_hash}. Error: {error}")]
    BlockSignatureVerificationFailed { block_hash: BlockHash, error: CryptoError },
}

/// Verifies that the the block header was signed by the expected sequencer.
pub fn verify_block_signature(
    sequencer_pub_key: &SequencerPublicKey,
    signature: &BlockSignature,
    state_diff_commitment: &GlobalRoot,
    block_hash: &BlockHash,
) -> Result<bool, BlockVerificationError> {
    let message_hash = poseidon_hash_array(&[block_hash.0, state_diff_commitment.0]);
    verify_message_hash_signature(&message_hash.0, &signature.0, &sequencer_pub_key.0).map_err(
        |err| BlockVerificationError::BlockSignatureVerificationFailed {
            block_hash: *block_hash,
            error: err,
        },
    )
}
