use crate::block_hash::utils::hash_array_preimage;
use crate::hash::{poseidon_hash_array, PoseidonHash};
use crate::transaction::{Event, TransactionHash};

#[cfg(test)]
#[path = "event_hash_test.rs"]
mod event_hash_test;

/// Poseidon(
///    from_address, transaction_hash,
///    num_keys, key0, key1, ...,
///    num_contents, content0, content1, ...
/// ).
pub fn calculate_event_hash(event: &Event, transaction_hash: &TransactionHash) -> PoseidonHash {
    let keys = &event.content.keys.iter().map(|k| k.0).collect::<Vec<_>>();
    let data = &event.content.data.0;
    poseidon_hash_array(
        &[
            vec![*event.from_address.0.key(), transaction_hash.0],
            hash_array_preimage(keys),
            hash_array_preimage(data),
        ]
        .concat(),
    )
}
