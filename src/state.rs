#[cfg(test)]
#[path = "state_test.rs"]
mod state_test;

#[cfg(feature = "std")]
use std::collections::hash_map::RandomState as HasherBuilder;

#[cfg(not(feature = "std"))]
use hashbrown::hash_map::DefaultHashBuilder as HasherBuilder;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::api_core::{
    ClassHash, ContractAddress, GlobalRoot, Nonce, PatriciaKey,
};
use crate::block::{BlockHash, BlockNumber};
use crate::hash::{StarkFelt, StarkHash};
use crate::deprecated_contract_class::ContractClass;

use crate::StarknetApiError;


/// The differences between two states before and after a block with hash block_hash
/// and their respective roots.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct StateUpdate {
    pub block_hash: BlockHash,
    pub new_root: GlobalRoot,
    pub old_root: GlobalRoot,
    pub state_diff: StateDiff,
}

/// The differences between two states.
// Invariant: Addresses are strictly increasing.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct StateDiff {
    pub deployed_contracts: IndexMap<ContractAddress, ClassHash, HasherBuilder>,
    pub storage_diffs:
        IndexMap<ContractAddress, IndexMap<StorageKey, StarkFelt, HasherBuilder>, HasherBuilder>,
    pub deprecated_declared_classes: IndexMap<ClassHash, ContractClass, HasherBuilder>,
    pub nonces: IndexMap<ContractAddress, Nonce, HasherBuilder>,
}

/// The sequential numbering of the states between blocks.
// Example:
// States: S0       S1       S2
// Blocks      B0->     B1->
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct StateNumber(pub BlockNumber);

impl StateNumber {
    /// The state at the beginning of the block.
    pub fn right_before_block(block_number: BlockNumber) -> StateNumber {
        StateNumber(block_number)
    }

    /// The state at the end of the block.
    pub fn right_after_block(block_number: BlockNumber) -> StateNumber {
        StateNumber(block_number.next())
    }

    pub fn is_before(&self, block_number: BlockNumber) -> bool {
        self.0 <= block_number
    }

    pub fn is_after(&self, block_number: BlockNumber) -> bool {
        !self.is_before(block_number)
    }

    pub fn block_after(&self) -> BlockNumber {
        self.0
    }
}

/// A storage key in a contract.
#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct StorageKey(pub PatriciaKey);

impl TryFrom<StarkHash> for StorageKey {
    type Error = StarknetApiError;

    fn try_from(val: StarkHash) -> Result<Self, Self::Error> {
        Ok(Self(PatriciaKey::try_from(val)?))
    }
}
