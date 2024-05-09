use super::event_commitment::{calculate_events_commitment, EventLeafElement};
use super::receipt_commitment::{calculate_receipt_commitment, ReceiptElement};
use super::state_diff_hash::calculate_state_diff_hash;
use super::transaction_commitment::{calculate_transactions_commitment, TransactionLeafElement};
use crate::block::GasPricePerToken;
use crate::core::{EventCommitment, ReceiptCommitment, StateDiffCommitment, TransactionCommitment};
use crate::data_availability::L1DataAvailabilityMode;
use crate::hash::{PoseidonHashCalculator, StarkFelt};
use crate::state::ThinStateDiff;
use crate::transaction::{
    TransactionHash, TransactionOutput, TransactionSignature, TransactionVersion,
};

#[cfg(test)]
#[path = "block_hash_calculator_test.rs"]
mod block_hash_calculator_test;

pub struct TransactionHashingData {
    pub transaction_signature: Option<TransactionSignature>,
    pub transaction_output: TransactionOutput,
    pub transaction_hash: TransactionHash,
    pub transaction_version: TransactionVersion,
}

#[allow(dead_code)]
struct BlockHeaderCommitments {
    pub n_transactions: usize,
    pub transactions_commitment: TransactionCommitment,
    pub n_events: usize,
    pub events_commitment: EventCommitment,
    pub receipts_commitment: ReceiptCommitment,
    pub state_diff_hash: StateDiffCommitment,
}

// Calculates the commitments of the transactions data for the block hash.
#[allow(dead_code)]
fn calculate_block_commitments(
    transactions_data: &[TransactionHashingData],
    state_diff: &ThinStateDiff,
    l1_data_gas_price_per_token: GasPricePerToken,
    l1_gas_price_per_token: GasPricePerToken,
) -> BlockHeaderCommitments {
    let transaction_leaf_elements: Vec<TransactionLeafElement> = transactions_data
        .iter()
        .map(|transaction_data| TransactionLeafElement {
            transaction_hash: transaction_data.transaction_hash,
            transaction_signature: transaction_data.transaction_signature.clone(),
        })
        .collect();
    let transactions_commitment =
        calculate_transactions_commitment::<PoseidonHashCalculator>(&transaction_leaf_elements);

    let mut event_leaf_elements: Vec<EventLeafElement> = Vec::new();
    for transaction in transactions_data {
        let transaction_hash = transaction.transaction_hash;
        for event in transaction.transaction_output.events() {
            let event_leaf_element = EventLeafElement { event: event.clone(), transaction_hash };
            event_leaf_elements.push(event_leaf_element);
        }
    }
    let events_commitment =
        calculate_events_commitment::<PoseidonHashCalculator>(&event_leaf_elements);

    let receipt_elements: Vec<ReceiptElement> = transactions_data
        .iter()
        .map(|transaction_data| ReceiptElement {
            transaction_hash: transaction_data.transaction_hash,
            transaction_output: transaction_data.transaction_output.clone(),
            transaction_version: transaction_data.transaction_version,
        })
        .collect();
    let receipts_commitment = calculate_receipt_commitment::<PoseidonHashCalculator>(
        &receipt_elements,
        l1_data_gas_price_per_token,
        l1_gas_price_per_token,
    );
    let state_diff_hash = calculate_state_diff_hash(state_diff);
    BlockHeaderCommitments {
        n_transactions: transactions_data.len(),
        transactions_commitment,
        n_events: event_leaf_elements.len(),
        events_commitment,
        receipts_commitment,
        state_diff_hash,
    }
}

// A single felt: [
//     transaction_count (64 bits) | event_count (64 bits) | state_diff_length (64 bits)
//     | L1 data availability mode: 0 for calldata, 1 for blob (1 bit) | 0 ...
// ].
#[allow(dead_code)]
fn concat_counts(
    transaction_count: usize,
    event_count: usize,
    state_diff_length: usize,
    l1_data_availability_mode: L1DataAvailabilityMode,
) -> StarkFelt {
    let l1_data_availability_byte: u8 = match l1_data_availability_mode {
        L1DataAvailabilityMode::Calldata => 0,
        L1DataAvailabilityMode::Blob => 0b10000000,
    };
    let concat_bytes = [
        to_64_bits(transaction_count).as_slice(),
        to_64_bits(event_count).as_slice(),
        to_64_bits(state_diff_length).as_slice(),
        &[l1_data_availability_byte],
        &[0_u8; 7], // zero padding
    ]
    .concat();
    StarkFelt::new_unchecked(concat_bytes.try_into().expect("Expect 32 bytes"))
}

fn to_64_bits(num: usize) -> [u8; 8] {
    let sized_transaction_count: u64 = num.try_into().expect("Expect usize is at most 8 bytes");
    sized_transaction_count.to_be_bytes()
}
