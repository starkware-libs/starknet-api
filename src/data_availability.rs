use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;

use crate::StarknetApiError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DataAvailabilityMode {
    L1 = 0,
    L2 = 1,
}

impl TryFrom<Felt> for DataAvailabilityMode {
    type Error = StarknetApiError;

    fn try_from(felt: Felt) -> Result<Self, StarknetApiError> {
        if felt == Felt::ZERO {
            Ok(DataAvailabilityMode::L1)
        } else if felt == Felt::ONE {
            Ok(DataAvailabilityMode::L2)
        } else {
            Err(StarknetApiError::OutOfRange {
                string: format!("Invalid data availability mode: {felt}."),
            })
        }
    }
}

impl From<DataAvailabilityMode> for Felt {
    fn from(data_availability_mode: DataAvailabilityMode) -> Felt {
        match data_availability_mode {
            DataAvailabilityMode::L1 => Felt::ZERO,
            DataAvailabilityMode::L2 => Felt::ONE,
        }
    }
}
