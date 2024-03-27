#[cfg(test)]
#[path = "block_hash_test.rs"]
mod block_hash_test;

use starknet_types_core::felt::Felt;
use starknet_types_core::hash::{Pedersen, StarkHash};

use crate::hash::StarkFelt;
use crate::transaction::Event;
use crate::StarknetApiError;

pub fn calculate_event_hash(event: &Event) -> Result<StarkFelt, StarknetApiError> {
    let keys_hash = Pedersen::hash_array(
        &event
            .content
            .keys
            .iter()
            .map(|key| Felt::from_bytes_be(key.0.bytes()))
            .collect::<Vec<Felt>>(),
    );
    let data_hash = Pedersen::hash_array(
        &event
            .content
            .data
            .0
            .iter()
            .map(|key| Felt::from_bytes_be(key.bytes()))
            .collect::<Vec<Felt>>(),
    );
    let event_hash = Pedersen::hash_array(&[
        Felt::from_bytes_be(event.from_address.0.key().bytes()),
        keys_hash,
        data_hash,
    ]);
    StarkFelt::new(event_hash.to_bytes_be())
}
