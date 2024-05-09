use crate::core::EventCommitment;
use crate::crypto::patricia_hash::calculate_root;
use crate::crypto::utils::HashChain;
use crate::hash::{HashFunction, StarkFelt};
use crate::transaction::{Event, TransactionHash};

#[cfg(test)]
#[path = "event_commitment_test.rs"]
mod event_commitment_test;

/// The elements used to calculate a leaf in the transactions Patricia tree.
#[derive(Clone)]
pub struct EventLeafElement {
    event: Event,
    transaction_hash: TransactionHash,
}

/// Returns the root of a Patricia tree where each leaf is an event hash.
pub fn calculate_events_commitment<H: HashFunction>(
    event_leaf_elements: &[EventLeafElement],
) -> EventCommitment {
    let event_leaves = event_leaf_elements.iter().map(calculate_event_hash).collect();
    EventCommitment(calculate_root::<H>(event_leaves))
}

// Poseidon(
//    from_address, transaction_hash,
//    num_keys, key0, key1, ...,
//    num_contents, content0, content1, ...
// ).
fn calculate_event_hash(event_leaf_element: &EventLeafElement) -> StarkFelt {
    let keys =
        &event_leaf_element.event.content.keys.iter().map(|k| k.0).collect::<Vec<StarkFelt>>();
    let data = &event_leaf_element.event.content.data.0;
    HashChain::new()
        .chain(event_leaf_element.event.from_address.0.key())
        .chain(&event_leaf_element.transaction_hash.0)
        .chain_size_and_elements(keys)
        .chain_size_and_elements(data)
        .get_poseidon_hash()
}
