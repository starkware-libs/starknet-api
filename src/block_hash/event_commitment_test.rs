use super::calculate_event_hash;
use crate::block_hash::event_commitment::calculate_events_commitment;
use crate::block_hash::test_utils::get_event_leaf_element;
use crate::core::EventCommitment;
use crate::hash::{PoseidonHashCalculator, StarkFelt};

#[test]
fn test_events_commitment_regression() {
    let event_leaf_elements =
        [get_event_leaf_element(0), get_event_leaf_element(1), get_event_leaf_element(2)];

    let expected_root =
        StarkFelt::try_from("0x069bb140ddbbeb01d81c7201ecfb933031306e45dab9c77ff9f9ba3cd4c2b9c3")
            .unwrap();

    assert_eq!(
        EventCommitment(expected_root),
        calculate_events_commitment::<PoseidonHashCalculator>(&event_leaf_elements),
    );
}

#[test]
fn test_event_hash_regression() {
    let event_leaf_element = get_event_leaf_element(2);

    let expected_hash =
        StarkFelt::try_from("0x367807f532742a4dcbe2d8a47b974b22dd7496faa75edc64a3a5fdb6709057")
            .unwrap();

    assert_eq!(expected_hash, calculate_event_hash(&event_leaf_element));
}
