use crate::data_availability::L1DataAvailabilityMode;
use crate::hash::StarkFelt;
use crate::StarknetApiError;

#[cfg(test)]
#[path = "block_hash_calculator_test.rs"]
mod block_hash_calculator_test;

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
) -> Result<StarkFelt, StarknetApiError> {
    let l1_data_availability_byte: u8 = match l1_data_availability_mode {
        L1DataAvailabilityMode::Calldata => 0,
        L1DataAvailabilityMode::Blob => 0b10000000,
    };
    let concat_bytes = [
        to_64_bits(transaction_count, "Too many transactions".to_owned())?.as_slice(),
        to_64_bits(event_count, "Too many events".to_owned())?.as_slice(),
        to_64_bits(state_diff_length, "Too long state diff".to_owned())?.as_slice(),
        &[l1_data_availability_byte],
        &[0_u8; 7], // zero padding
    ]
    .concat();
    StarkFelt::new(concat_bytes.try_into().expect("Expect 32 bytes"))
}

fn to_64_bits(num: usize, error_message: String) -> Result<[u8; 8], StarknetApiError> {
    let sized_transaction_count: u64 =
        num.try_into().map_err(|_| StarknetApiError::OutOfRange { string: error_message })?;
    Ok(sized_transaction_count.to_be_bytes())
}
