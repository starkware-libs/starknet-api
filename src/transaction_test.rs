use strum::IntoEnumIterator;

use crate::transaction::Builtin;

#[test]
fn test_builtin_enum_order() {
    let expected_builtin_order = [
        // Builtins that are allowed for direct use by contracts on Starknet.
        Builtin::Pedersen,
        Builtin::RangeCheck,
        Builtin::Ecdsa,
        Builtin::Bitwise,
        Builtin::EcOp,
        Builtin::Poseidon,
        Builtin::SegmentArena,
        // Builtins that are not allowed for direct use by contracts on Starknet (They are allowed
        // for indirect use, for example, by syscall).
        Builtin::Keccak,
    ];
    let from_iter = Builtin::iter().collect::<Vec<Builtin>>();
    assert_eq!(&from_iter, &expected_builtin_order);
}
