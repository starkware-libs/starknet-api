use once_cell::sync::Lazy;

use super::event_commitment::{calculate_events_commitment, EventLeafElement};
use super::receipt_commitment::calculate_receipt_commitment;
use super::state_diff_hash::calculate_state_diff_hash;
use super::transaction_commitment::{calculate_transactions_commitment, TransactionLeafElement};
use crate::block::{BlockHash, BlockNumber, BlockTimestamp, GasPricePerToken, StarknetVersion};
use crate::core::{GlobalRoot, SequencerContractAddress};
use crate::crypto::utils::HashChain;
use crate::data_availability::L1DataAvailabilityMode;
use crate::hash::{PoseidonHashCalculator, StarkFelt};
use crate::state::ThinStateDiff;
use crate::transaction::TransactionReceipt;
use crate::transaction_hash::ascii_as_felt;

#[cfg(test)]
#[path = "block_hash_calculator_test.rs"]
mod block_hash_calculator_test;

static STARKNET_BLOCK_HASH0: Lazy<StarkFelt> = Lazy::new(|| {
    ascii_as_felt("STARKNET_BLOCK_HASH0").expect("ascii_as_felt failed for 'STARKNET_BLOCK_HASH0'")
});

pub struct BlockHashInput<'a, 'b, 'c, 'd> {
    block_number: BlockNumber,
    state_root: GlobalRoot,
    sequencer_contract_address: SequencerContractAddress,
    timestamp: BlockTimestamp,
    transaction_leaf_elements: &'a [TransactionLeafElement],
    event_leaf_elements: &'b [EventLeafElement],
    state_diff: &'c ThinStateDiff,
    l1_da_mode: L1DataAvailabilityMode,
    transaction_receipts: &'d [TransactionReceipt],
    l1_gas_price: GasPricePerToken,
    l1_data_gas_price: GasPricePerToken,
    starknet_version: StarknetVersion,
    parent_hash: BlockHash,
}

/// Poseidon (
///     “STARKNET_BLOCK_HASH0”, block_number, global_state_root, sequencer_address,
///     block_timestamp, concat_counts, state_diff_hash, transaction_commitment,
///     event_commitment, receipt_commitment, gas_price_wei, gas_price_fri,
///     data_gas_price_wei, data_gas_price_fri, starknet_version, 0, parent_block_hash
/// ).
pub fn calculate_block_hash(block_hash_input: &BlockHashInput<'_, '_, '_, '_>) -> BlockHash {
    let transactions_commitment = calculate_transactions_commitment::<PoseidonHashCalculator>(
        block_hash_input.transaction_leaf_elements,
    );
    let events_commitment =
        calculate_events_commitment::<PoseidonHashCalculator>(block_hash_input.event_leaf_elements);
    let receipt_commitment = calculate_receipt_commitment::<PoseidonHashCalculator>(
        block_hash_input.transaction_receipts,
    );

    BlockHash(
        HashChain::new()
            .chain(&STARKNET_BLOCK_HASH0)
            .chain(&block_hash_input.block_number.0.into())
            .chain(&block_hash_input.state_root.0)
            .chain(&block_hash_input.sequencer_contract_address.0)
            .chain(&block_hash_input.timestamp.0.into())
            .chain(&concat_counts(
                block_hash_input.transaction_leaf_elements.len(),
                block_hash_input.event_leaf_elements.len(),
                block_hash_input.state_diff.len(),
                block_hash_input.l1_da_mode,
            ))
            .chain(&calculate_state_diff_hash(block_hash_input.state_diff).0.0)
            .chain(&transactions_commitment.0)
            .chain(&events_commitment.0)
            .chain(&receipt_commitment.0)
            .chain(&block_hash_input.l1_gas_price.price_in_wei.0.into())
            .chain(&block_hash_input.l1_gas_price.price_in_fri.0.into())
            .chain(&block_hash_input.l1_data_gas_price.price_in_wei.0.into())
            .chain(&block_hash_input.l1_data_gas_price.price_in_fri.0.into())
            .chain(
                &ascii_as_felt(&block_hash_input.starknet_version.0).expect("Expect ASCII version"),
            )
            .chain(&StarkFelt::ZERO)
            .chain(&block_hash_input.parent_hash.0)
            .get_poseidon_hash(),
    )
}

// A single felt: [
//     transaction_count (64 bits) | event_count (64 bits) | state_diff_length (64 bits)
//     | L1 data availability mode: 0 for calldata, 1 for blob (1 bit) | 0 ...
// ].
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
