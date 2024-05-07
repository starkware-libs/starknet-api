use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::core::{ClassHash, CompiledClassHash, ContractAddress, StateDiffCommitment};
use crate::crypto::utils::HashChain;
use crate::hash::{PoseidonHash, StarkFelt};
use crate::state::{StorageKey, ThinStateDiff};
use crate::transaction_hash::ascii_as_felt;

#[cfg(test)]
#[path = "state_diff_hash_test.rs"]
mod state_diff_hash_test;

static STARKNET_STATE_DIFF0: Lazy<StarkFelt> = Lazy::new(|| {
    ascii_as_felt("STARKNET_STATE_DIFF0").expect("ascii_as_felt failed for 'STARKNET_STATE_DIFF0'")
});

/// Poseidon(
///     "STARKNET_STATE_DIFF0", deployed_contracts, declared_classes, deprecated_declared_classes,
///     DA_modes
/// ).
pub fn calculate_state_diff_hash(state_diff: &ThinStateDiff) -> StateDiffCommitment {
    let mut hash_chain = HashChain::new().chain(&STARKNET_STATE_DIFF0);
    hash_chain = chain_deployed_contracts(&state_diff.deployed_contracts, hash_chain);
    hash_chain = chain_declared_classes(&state_diff.declared_classes, hash_chain);
    hash_chain =
        chain_deprecated_declared_classes(&state_diff.deprecated_declared_classes, hash_chain);
    hash_chain = chain_data_availability_modes(&state_diff.storage_diffs, hash_chain);
    StateDiffCommitment(PoseidonHash(hash_chain.get_poseidon_hash()))
}

// Chains: [number_of_deployed_contracts, address_0, class_hash_0, address_1, class_hash_1, ...].
fn chain_deployed_contracts(
    deployed_contracts: &IndexMap<ContractAddress, ClassHash>,
    mut hash_chain: HashChain,
) -> HashChain {
    hash_chain = hash_chain.chain(&deployed_contracts.len().into());
    for (address, class_hash) in deployed_contracts {
        hash_chain = hash_chain.chain(address).chain(class_hash);
    }
    hash_chain
}

// Chains: [number_of_declared_classes,
//      class_hash_0, compiled_class_hash_0, class_hash_1, compiled_class_hash_1, ...].
fn chain_declared_classes(
    declared_classes: &IndexMap<ClassHash, CompiledClassHash>,
    mut hash_chain: HashChain,
) -> HashChain {
    hash_chain = hash_chain.chain(&declared_classes.len().into());
    for (class_hash, compiled_class_hash) in declared_classes {
        hash_chain = hash_chain.chain(class_hash).chain(&compiled_class_hash.0)
    }
    hash_chain
}

// Chains: [number_of_old_declared_classes, class_hash_0, class_hash_1, ...].
fn chain_deprecated_declared_classes(
    deprecated_declared_classes: &[ClassHash],
    hash_chain: HashChain,
) -> HashChain {
    hash_chain
        .chain(&deprecated_declared_classes.len().into())
        .chain_iter(deprecated_declared_classes.iter().map(|class_hash| &class_hash.0))
}

// Chains: [number_of_DA_modes,
//      DA_mode_0, storage_domain_state_diff_0, DA_mode_1, storage_diff_1, ...].
// Currently, only DA_mode=0 is in use.
fn chain_data_availability_modes(
    storage_diffs: &IndexMap<ContractAddress, IndexMap<StorageKey, StarkFelt>>,
    mut hash_chain: HashChain,
) -> HashChain {
    hash_chain = hash_chain.chain(&StarkFelt::ONE) // number of DA modes.
        .chain(&StarkFelt::ZERO); // the DA mode.
    chain_storage_diffs(storage_diffs, hash_chain)
}

// Chains: [number_of_updated_contracts,
//      contract_address_0, number_of_updates_in_contract_0, key_0, value0, key1, value1, ...,
//      contract_address_1, number_of_updates_in_contract_1, key_0, value0, key1, value1, ...,
// ]
fn chain_storage_diffs(
    storage_diffs: &IndexMap<ContractAddress, IndexMap<StorageKey, StarkFelt>>,
    mut hash_chain: HashChain,
) -> HashChain {
    hash_chain = hash_chain.chain(&storage_diffs.len().into());
    for (contract_address, key_value_map) in storage_diffs {
        hash_chain = hash_chain.chain(contract_address);
        hash_chain = hash_chain.chain(&key_value_map.len().into());
        for (key, value) in key_value_map {
            hash_chain = hash_chain.chain(key).chain(value);
        }
    }
    hash_chain
}
