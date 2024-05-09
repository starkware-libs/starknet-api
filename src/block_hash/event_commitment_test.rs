use super::{calculate_event_hash, EventLeafElement};
use crate::block_hash::event_commitment::calculate_events_commitment;
use crate::core::{ContractAddress, EventCommitment, PatriciaKey};
use crate::hash::{PoseidonHashCalculator, StarkFelt, StarkHash};
use crate::transaction::{Event, EventContent, EventData, EventKey, TransactionHash};
use crate::{contract_address, patricia_key, stark_felt};

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

fn get_event_leaf_element(seed: u8) -> EventLeafElement {
    EventLeafElement {
        event: Event {
            from_address: contract_address!(seed + 8),
            content: EventContent {
                keys: [seed, seed + 1].iter().map(|key| EventKey(stark_felt!(*key))).collect(),
                data: EventData(
                    [seed + 2, seed + 3, seed + 4].into_iter().map(StarkFelt::from).collect(),
                ),
            },
        },
        transaction_hash: TransactionHash(stark_felt!("0x1234")),
    }
}
