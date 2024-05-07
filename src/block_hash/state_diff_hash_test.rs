use crate::block_hash::state_diff_hash::calculate_state_diff_hash;
use crate::block_hash::test_utils::get_state_diff;
use crate::core::StateDiffCommitment;
use crate::hash::{PoseidonHash, StarkFelt};

#[test]
fn test_state_diff_hash_regression() {
    let state_diff = get_state_diff();

    let expected_hash = StateDiffCommitment(PoseidonHash(
        StarkFelt::try_from("0x05b8241020c186585f4273cf991d35ad703e808bd9b40242cec584e7f2d86495")
            .unwrap(),
    ));

    assert_eq!(expected_hash, calculate_state_diff_hash(&state_diff));
}
