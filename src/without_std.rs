#[macro_use]
pub extern crate alloc;

pub mod without_std {
    pub use core::fmt;
    pub use core::hash;
    pub use core::mem;
    pub use core::num;

    pub use alloc::boxed;
    pub use alloc::format;
    pub use alloc::rc;
    pub use alloc::string;
    pub use alloc::sync;
    pub use alloc::vec;

    pub mod collections {
        pub use alloc::collections::BTreeMap;
        pub use hashbrown::{HashMap, HashSet};
    }

    pub mod borrow {
        pub use alloc::borrow::*;
        pub use core::borrow::*;
    }
}