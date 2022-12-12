use crate::core::PatriciaKey;
use crate::hash::StarkHash;
use crate::{patky, shash};

pub trait GetTestInstance: Sized {
    fn get_test_instance() -> Self;
}

////////////////////////////////////////////////////////////////////////
// Implements the [`GetTestInstance`] trait for primitive types
// that don't derive [`GetTestInstance`].
////////////////////////////////////////////////////////////////////////
impl_get_test_instance! {
    u64;
    u128;
}

#[macro_export]
macro_rules! impl_get_test_instance {
    () => {};
    ($name:ident; $($rest:tt)*) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self::default()
            }
        }
        impl_get_test_instance!($($rest)*);
    }
}
pub use impl_get_test_instance;

////////////////////////////////////////////////////////////////////////
// Implements the [`GetTestInstance`] trait for starknet_api types
// that don't derive [`GetTestInstance`].
////////////////////////////////////////////////////////////////////////
impl GetTestInstance for StarkHash {
    fn get_test_instance() -> Self {
        shash!("0x1")
    }
}

impl GetTestInstance for PatriciaKey {
    fn get_test_instance() -> Self {
        patky!("0x1")
    }
}
