use starknet_types_core::felt::Felt;

use super::calculate_event_hash;
use crate::core::{ContractAddress, EventCommitment, PatriciaKey};
use crate::hash::StarkHash;
use crate::transaction::{Event, EventContent, EventData, EventKey, TransactionHash};
use crate::{contract_address, patricia_key};

#[test]
fn test_event_hash_regression() {
    let event = Event {
        from_address: contract_address!("0xA"),
        content: EventContent {
            keys: [2_u8, 3].iter().map(|key| EventKey(Felt::from(*key))).collect(),
            data: EventData([4_u8, 5, 6].into_iter().map(Felt::from).collect()),
        },
    };
    let tx_hash = TransactionHash(Felt::from_hex_unchecked("0x1234"));

    let expected_hash = EventCommitment(Felt::from_hex_unchecked(
        "0x367807f532742a4dcbe2d8a47b974b22dd7496faa75edc64a3a5fdb6709057",
    ));

    assert_eq!(expected_hash, calculate_event_hash(&event, &tx_hash));
}
