use indexmap::indexmap;

use crate::block_hash::state_diff_hash::{
    calculate_state_diff_hash, chain_declared_classes, chain_deployed_contracts,
    chain_deprecated_declared_classes, chain_nonces, chain_storage_diffs,
};
use crate::core::{ClassHash, CompiledClassHash, Nonce, StateDiffCommitment};
use crate::crypto::utils::HashChain;
use crate::hash::{PoseidonHash, StarkFelt};
use crate::state::ThinStateDiff;

#[test]
fn test_state_diff_hash_regression() {
    let state_diff = ThinStateDiff {
        deployed_contracts: indexmap! {
            0u64.into() => ClassHash(1u64.into()),
            2u64.into() => ClassHash(3u64.into()),
        },
        storage_diffs: indexmap! {
            4u64.into() => indexmap! {
                5u64.into() => 6u64.into(),
                7u64.into() => 8u64.into(),
            },
            9u64.into() => indexmap! {
                10u64.into() => 11u64.into(),
            },
        },
        declared_classes: indexmap! {
            ClassHash(12u64.into()) => CompiledClassHash(13u64.into()),
            ClassHash(14u64.into()) => CompiledClassHash(15u64.into()),
        },
        deprecated_declared_classes: vec![ClassHash(16u64.into())],
        nonces: indexmap! {
            17u64.into() => Nonce(18u64.into()),
        },
        replaced_classes: indexmap! {
            19u64.into() => ClassHash(20u64.into()),
        },
    };

    let expected_hash = StateDiffCommitment(PoseidonHash(
        StarkFelt::try_from("0x05b8241020c186585f4273cf991d35ad703e808bd9b40242cec584e7f2d86495")
            .unwrap(),
    ));

    assert_eq!(expected_hash, calculate_state_diff_hash(&state_diff));
}

#[test]
fn test_sorting_deployed_contracts() {
    let deployed_contracts_0 = indexmap! {
        0u64.into() => ClassHash(3u64.into()),
        1u64.into() => ClassHash(2u64.into()),
    };
    let deployed_contracts_1 = indexmap! {
        1u64.into() => ClassHash(2u64.into()),
        0u64.into() => ClassHash(3u64.into()),
    };
    assert_eq!(
        chain_deployed_contracts(&deployed_contracts_0, HashChain::new()).get_poseidon_hash(),
        chain_deployed_contracts(&deployed_contracts_1, HashChain::new()).get_poseidon_hash(),
    );
}

#[test]
fn test_sorting_declared_classes() {
    let declared_classes_0 = indexmap! {
        ClassHash(0u64.into()) => CompiledClassHash(3u64.into()),
        ClassHash(1u64.into()) => CompiledClassHash(2u64.into()),
    };
    let declared_classes_1 = indexmap! {
        ClassHash(1u64.into()) => CompiledClassHash(2u64.into()),
        ClassHash(0u64.into()) => CompiledClassHash(3u64.into()),
    };
    assert_eq!(
        chain_declared_classes(&declared_classes_0, HashChain::new()).get_poseidon_hash(),
        chain_declared_classes(&declared_classes_1, HashChain::new()).get_poseidon_hash(),
    );
}

#[test]
fn test_sorting_deprecated_declared_classes() {
    let deprecated_declared_classes_0 = vec![ClassHash(0u64.into()), ClassHash(1u64.into())];
    let deprecated_declared_classes_1 = vec![ClassHash(1u64.into()), ClassHash(0u64.into())];
    assert_eq!(
        chain_deprecated_declared_classes(&deprecated_declared_classes_0, HashChain::new())
            .get_poseidon_hash(),
        chain_deprecated_declared_classes(&deprecated_declared_classes_1, HashChain::new())
            .get_poseidon_hash(),
    );
}

#[test]
fn test_sorting_storage_diffs() {
    let storage_diffs_0 = indexmap! {
        0u64.into() => indexmap! {
            1u64.into() => 2u64.into(),
            3u64.into() => 4u64.into(),
        },
        5u64.into() => indexmap! {
            6u64.into() => 7u64.into(),
        },
    };
    let storage_diffs_1 = indexmap! {
        5u64.into() => indexmap! {
            6u64.into() => 7u64.into(),
        },
        0u64.into() => indexmap! {
            3u64.into() => 4u64.into(),
            1u64.into() => 2u64.into(),
        },
    };
    assert_eq!(
        chain_storage_diffs(&storage_diffs_0, HashChain::new()).get_poseidon_hash(),
        chain_storage_diffs(&storage_diffs_1, HashChain::new()).get_poseidon_hash(),
    );
}

#[test]
fn test_empty_storage_diffs() {
    let storage_diffs_0 = indexmap! {
        0u64.into() => indexmap! {
            1u64.into() => 2u64.into(),
        },
        3u64.into() => indexmap! {
        },
    };
    let storage_diffs_1 = indexmap! {
        0u64.into() => indexmap! {
            1u64.into() => 2u64.into(),
        },
    };
    assert_eq!(
        chain_storage_diffs(&storage_diffs_0, HashChain::new()).get_poseidon_hash(),
        chain_storage_diffs(&storage_diffs_1, HashChain::new()).get_poseidon_hash(),
    );
}

#[test]
fn test_sorting_nonces() {
    let nonces_0 = indexmap! {
        0u64.into() => Nonce(3u64.into()),
        1u64.into() => Nonce(2u64.into()),
    };
    let nonces_1 = indexmap! {
        1u64.into() => Nonce(2u64.into()),
        0u64.into() => Nonce(3u64.into()),
    };
    assert_eq!(
        chain_nonces(&nonces_0, HashChain::new()).get_poseidon_hash(),
        chain_nonces(&nonces_1, HashChain::new()).get_poseidon_hash(),
    );
}
