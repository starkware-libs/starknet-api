pub mod with_std {
    pub use std::{borrow, fmt, mem, string, sync, vec};

    pub mod collections {
        pub use std::collections::HashMap;
    }
}
