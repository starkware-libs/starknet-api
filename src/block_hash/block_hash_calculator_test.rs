use super::concat_counts;
use crate::block::{
    BlockHash, BlockNumber, BlockTimestamp, GasPrice, GasPricePerToken, StarknetVersion,
};
use crate::block_hash::block_hash_calculator::{calculate_block_hash, BlockHashInput};
use crate::block_hash::test_utils::{
    get_event_leaf_element, get_state_diff, get_transaction_leaf_element, get_transaction_reciept,
};
use crate::core::{ContractAddress, GlobalRoot, PatriciaKey, SequencerContractAddress};
use crate::data_availability::L1DataAvailabilityMode;
use crate::hash::StarkFelt;

#[test]
fn test_block_hash_regression() {
    let block_hash_input = BlockHashInput {
        block_number: BlockNumber(1_u64),
        state_root: GlobalRoot(StarkFelt::from(2_u8)),
        sequencer_contract_address: SequencerContractAddress(ContractAddress(PatriciaKey::from(
            3_u8,
        ))),
        timestamp: BlockTimestamp(4),
        transaction_leaf_elements: &[get_transaction_leaf_element()],
        event_leaf_elements: &[get_event_leaf_element(5)],
        state_diff: &get_state_diff(),
        l1_da_mode: L1DataAvailabilityMode::Blob,
        transaction_receipts: &[get_transaction_reciept()],
        l1_gas_price: GasPricePerToken { price_in_fri: GasPrice(6), price_in_wei: GasPrice(7) },
        l1_data_gas_price: GasPricePerToken {
            price_in_fri: GasPrice(10),
            price_in_wei: GasPrice(9),
        },
        starknet_version: StarknetVersion("10".to_owned()),
        parent_hash: BlockHash(StarkFelt::from(11_u8)),
    };

    let expected_hash =
        StarkFelt::try_from("0x0516e9005367f060d2baf884ee39a9d29e8badb4a9be51222128e6e01f0ce759")
            .unwrap();

    assert_eq!(BlockHash(expected_hash), calculate_block_hash(&block_hash_input),);
}

#[test]
fn concat_counts_test() {
    let concated = concat_counts(4, 3, 2, L1DataAvailabilityMode::Blob);
    let expected_felt =
        StarkFelt::try_from("0x0000000000000004000000000000000300000000000000028000000000000000")
            .unwrap();
    assert_eq!(concated, expected_felt)
}
