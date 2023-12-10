pub mod with_std {
    pub use std::borrow;
    pub use std::fmt;
    pub use std::format;
    pub use std::mem;
    pub use std::num;

    pub use std::string;
    pub use std::sync;
    pub use std::vec;

    pub mod collections {
        pub use std::collections::{BTreeMap, HashMap, HashSet};
    }
}