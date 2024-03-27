use crate::block_hash::calculate_event_hash;
use crate::core::{ContractAddress, PatriciaKey};
use crate::hash::{StarkFelt, StarkHash};
use crate::transaction::{Event, EventContent, EventData, EventKey};
use crate::{contract_address, patricia_key, stark_felt};

#[test]
fn test_event_hash() {
    let event = Event {
        from_address: contract_address!(10_u8),
        content: EventContent {
            keys: [2_u8, 3].iter().map(|key| EventKey(stark_felt!(*key))).collect(),
            data: EventData([4_u8, 5, 6].into_iter().map(StarkFelt::from).collect()),
        },
    };

    // Regression value taken from known working implementation.
    let expected_hash =
        StarkFelt::try_from("0x3f44fb0516121d225664058ecc7e415c4725d6a7a11fd7d515c55c34ef8270b")
            .unwrap();

    assert_eq!(expected_hash, calculate_event_hash(&event).unwrap());
}
