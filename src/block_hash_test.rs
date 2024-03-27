use crate::block_hash::calculate_event_hash;
use crate::core::{ContractAddress, PatriciaKey};
use crate::hash::{PoseidonHash, StarkFelt, StarkHash};
use crate::transaction::{Event, EventContent, EventData, EventKey};
use crate::{contract_address, patricia_key, stark_felt};

#[test]
fn test_event_hash_regression() {
    let event = Event {
        from_address: contract_address!(10_u8),
        content: EventContent {
            keys: [2_u8, 3].iter().map(|key| EventKey(stark_felt!(*key))).collect(),
            data: EventData([4_u8, 5, 6].into_iter().map(StarkFelt::from).collect()),
        },
    };

    let expected_hash = PoseidonHash(
        StarkFelt::try_from("0x058c90726c0e94f44d0bf7c80924ed8c2dfdc5cbd968da5013161e06053ea164")
            .unwrap(),
    );

    assert_eq!(expected_hash, calculate_event_hash(&event));
}
