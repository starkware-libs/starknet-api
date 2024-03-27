#[cfg(test)]
#[path = "block_hash_test.rs"]
mod block_hash_test;

use crate::hash::{poseidon_hash_array, PoseidonHash};
use crate::transaction::Event;

pub fn calculate_event_hash(event: &Event) -> PoseidonHash {
    let keys_hash =
        poseidon_hash_array(&event.content.keys.iter().map(|key| key.0).collect::<Vec<_>>());
    let data_hash = poseidon_hash_array(&event.content.data.0);
    poseidon_hash_array(&[*event.from_address.0.key(), keys_hash.0, data_hash.0])
}
