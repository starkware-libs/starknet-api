use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

pub type StarkHash = Felt;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct PoseidonHash(pub Felt);
