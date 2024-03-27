#[cfg(test)]
#[path = "block_hash_test.rs"]
mod block_hash_test;

use starknet_types_core::felt::Felt;

use crate::hash::{poseidon_hash_array, PoseidonHash};
use crate::transaction::{Event, TransactionHash};

pub fn calculate_event_hash(event: &Event, tx_hash: &TransactionHash) -> PoseidonHash {
    // Poseidon(
    //    from_address, transaction_hash,
    //    num_keys, key0, key1, ...,
    //    num_contents, content0, content1, ...
    // )
    poseidon_hash_array(
        &[
            &[*event.from_address.0.key(), tx_hash.0, Felt::from(event.content.keys.len()).into()],
            event.content.keys.iter().map(|k| k.0).collect::<Vec<_>>().as_slice(),
            &[Felt::from(event.content.data.0.len()).into()],
            event.content.data.0.as_slice(),
        ]
        .concat(),
    )
}
