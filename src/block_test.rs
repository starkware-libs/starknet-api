use crate::block::BlockNumber;
use crate::hash::StarkFelt;
use crate::{stark_felt, StarknetApiError};

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

#[test]
fn test_felt_try_into_block_number() {
    let value = u64::MAX;
    let felt: StarkFelt = stark_felt![value];
    let block_number: BlockNumber = felt.try_into().unwrap();
    assert_eq!(value, block_number.0);

    // Negative flow.
    let value: u128 = u64::MAX.into();
    let value = value + 1;
    let felt: StarkFelt = stark_felt![value];
    let err = BlockNumber::try_from(felt).unwrap_err();
    assert!(matches!(err, StarknetApiError::OutOfRange { .. }));
}
