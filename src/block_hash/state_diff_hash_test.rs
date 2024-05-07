use indexmap::indexmap;

use crate::block_hash::state_diff_hash::{calculate_state_diff_hash, state_diff_length};
use crate::core::{ClassHash, CompiledClassHash, Nonce, StateDiffCommitment};
use crate::hash::{PoseidonHash, StarkFelt};
use crate::state::ThinStateDiff;

#[test]
fn test_state_diff_length() {
    let state_diff = get_state_diff();
    assert_eq!(state_diff_length(&state_diff), 9);
}

#[test]
fn test_state_diff_hash_regression() {
    let state_diff = get_state_diff();

    let expected_hash = StateDiffCommitment(PoseidonHash(
        StarkFelt::try_from("0x01676de20d6689960498ea15b305212f9ef343b79e533b7b12ad618f22c17fd9")
            .unwrap(),
    ));

    assert_eq!(expected_hash, calculate_state_diff_hash(&state_diff));
}

fn get_state_diff() -> ThinStateDiff {
    ThinStateDiff {
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
    }
}
