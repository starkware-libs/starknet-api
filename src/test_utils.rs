use crate::block::{BlockHash, BlockHeader, BlockNumber, BlockTimestamp, GasPrice};
use crate::core::{ContractAddress, GlobalRoot, PatriciaKey};
use crate::hash::StarkHash;
use crate::{patky, shash};

pub trait GetTestInstance: Sized {
    fn get_test_instance() -> Self;
}

auto_impl_get_test_instance! {
    pub struct BlockHash(pub StarkHash);
    pub struct BlockHeader {
        pub block_hash: BlockHash,
        pub parent_hash: BlockHash,
        pub block_number: BlockNumber,
        pub gas_price: GasPrice,
        pub state_root: GlobalRoot,
        pub sequencer: ContractAddress,
        pub timestamp: BlockTimestamp,
    }
    pub struct BlockNumber(pub u64);
    pub struct BlockTimestamp(pub u64);
    pub struct GasPrice(pub u128);
    pub struct GlobalRoot(pub StarkHash);
    u64;
    u128;
}

#[macro_export]
macro_rules! auto_impl_get_test_instance {
    () => {};
    // Tuple structs (no names associated with fields) - one field.
    ($(pub)? struct $name:ident($(pub)? $ty:ty); $($rest:tt)*) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self(<$ty>::get_test_instance())
            }
        }
        auto_impl_get_test_instance!($($rest)*);
    };
    // Structs with public fields.
    ($(pub)? struct $name:ident { $(pub $field:ident : $ty:ty ,)* } $($rest:tt)*) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self {
                    $(
                        $field: <$ty>::get_test_instance(),
                    )*
                }
            }
        }
        auto_impl_get_test_instance!($($rest)*);
    };
    // Primitive types.
    ($name:ident; $($rest:tt)*) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self::default()
            }
        }
        auto_impl_get_test_instance!($($rest)*);
    }
}
pub use auto_impl_get_test_instance;

////////////////////////////////////////////////////////////////////////
// Implements the [`GetTestInstance`] trait for starknet_api types not
// supported by the macro [`auto_impl_get_test_instance`].
////////////////////////////////////////////////////////////////////////
impl GetTestInstance for StarkHash {
    fn get_test_instance() -> Self {
        shash!("0x1")
    }
}

impl GetTestInstance for ContractAddress {
    fn get_test_instance() -> Self {
        Self(patky!("0x1"))
    }
}
