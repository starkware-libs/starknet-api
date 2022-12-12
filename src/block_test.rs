use crate::block::BlockNumber;
#[cfg(feature = "testing")]
use crate::block::{BlockHash, BlockHeader, BlockTimestamp, GasPrice};
#[cfg(feature = "testing")]
use crate::core::{ContractAddress, GlobalRoot, PatriciaKey};
#[cfg(feature = "testing")]
use crate::hash::StarkHash;
#[cfg(feature = "testing")]
use crate::test_utils::GetTestInstance;
#[cfg(feature = "testing")]
use crate::{patky, shash};

#[test]
fn test_block_number_iteration() {
    let start: u64 = 3;
    let up_until: u64 = 10;

    let mut expected = vec![];
    for i in start..up_until {
        expected.push(BlockNumber(i));
    }

    let start_block_number = BlockNumber(start);
    let up_until_block_number = BlockNumber(up_until);

    let mut from_iter: Vec<_> = vec![];
    for i in start_block_number.iter_up_to(up_until_block_number) {
        from_iter.push(i);
    }

    assert_eq!(expected, from_iter);
}

#[cfg(feature = "testing")]
#[test]
fn test_get_test_instance() {
    let block_header = BlockHeader::get_test_instance();
    let expected_block_header = BlockHeader {
        block_hash: BlockHash(shash!("0x1")),
        parent_hash: BlockHash(shash!("0x1")),
        block_number: BlockNumber(0),
        gas_price: GasPrice(0),
        state_root: GlobalRoot(shash!("0x1")),
        sequencer: ContractAddress(patky!("0x1")),
        timestamp: BlockTimestamp(0),
    };
    assert_eq!(block_header, expected_block_header);
}
