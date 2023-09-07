use std::env;
use std::fs::read_to_string;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use super::{ascii_as_felt, get_transaction_hash, CONSTRUCTOR_ENTRY_POINT_SELECTOR};
use crate::core::ChainId;
use crate::hash::StarkFelt;
use crate::transaction::transaction_hash::validate_transaction_hash;
use crate::transaction::{Transaction, TransactionHash};

#[test]
fn test_ascii_as_felt() {
    let sn_main_id = ChainId("SN_MAIN".to_owned());
    let sn_main_felt = ascii_as_felt(sn_main_id.0.as_str()).unwrap();
    // This is the result of the Python snippet from the Chain-Id documentation.
    let expected_sn_main = StarkFelt::from(23448594291968334_u128);
    assert_eq!(sn_main_felt, expected_sn_main);
}

#[test]
fn test_constructor_selector() {
    let mut keccak = Keccak256::default();
    keccak.update(b"constructor");
    let mut constructor_bytes: [u8; 32] = keccak.finalize().into();
    constructor_bytes[0] &= 0b00000011_u8; // Discard the six MSBs.
    let constructor_felt = StarkFelt::new(constructor_bytes).unwrap();
    assert_eq!(constructor_felt, *CONSTRUCTOR_ENTRY_POINT_SELECTOR);
}

#[derive(Deserialize, Serialize)]
struct TransactionTestData {
    transaction: Transaction,
    transaction_hash: TransactionHash,
    chain_id: ChainId,
}

#[test]
fn test_transaction_hash() {
    // The details were taken from Starknet Goerli. You can found the transactions by hash in:
    // https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=<transaction_hash>
    let transactions_test_data_vec: Vec<TransactionTestData> =
        serde_json::from_value(read_json_file("transaction_hash.json")).unwrap();

    for transaction_test_data in transactions_test_data_vec {
        assert!(
            validate_transaction_hash(
                &transaction_test_data.transaction,
                &transaction_test_data.chain_id,
                transaction_test_data.transaction_hash
            )
            .unwrap()
        );
        let actual_transaction_hash = get_transaction_hash(
            &transaction_test_data.transaction,
            &transaction_test_data.chain_id,
        )
        .unwrap();
        assert_eq!(
            actual_transaction_hash, transaction_test_data.transaction_hash,
            "expected_transaction_hash: {:?}",
            transaction_test_data.transaction_hash
        );
    }
}

#[test]
fn test_deprecated_transaction_hash() {
    // The details were taken from Starknet Goerli. You can found the transactions by hash in:
    // https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=<transaction_hash>
    let transaction_test_data_vec: Vec<TransactionTestData> =
        serde_json::from_value(read_json_file("deprecated_transaction_hash.json")).unwrap();

    for transaction_test_data in transaction_test_data_vec {
        assert!(
            validate_transaction_hash(
                &transaction_test_data.transaction,
                &transaction_test_data.chain_id,
                transaction_test_data.transaction_hash
            )
            .unwrap(),
            "expected_transaction_hash: {:?}",
            transaction_test_data.transaction_hash
        );
    }
}

pub fn read_json_file(path_in_resource_dir: &str) -> serde_json::Value {
    let path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("resources")
        .join(path_in_resource_dir);
    let json_str = read_to_string(path.to_str().unwrap()).unwrap();
    serde_json::from_str(&json_str).unwrap()
}
