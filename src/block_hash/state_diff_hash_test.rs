use indexmap::indexmap;

use crate::block_hash::state_diff_hash::calculate_state_diff_hash;
use crate::core::{ClassHash, CompiledClassHash, Nonce, StateDiffCommitment};
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
