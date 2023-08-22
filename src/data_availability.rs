use serde::{Deserialize, Serialize};

use crate::hash::StarkFelt;
use crate::StarknetApiError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DataAvailabilityMode {
    L1 = 0,
    L2 = 1,
}

impl TryFrom<StarkFelt> for DataAvailabilityMode {
    type Error = StarknetApiError;

    fn try_from(felt: StarkFelt) -> Result<Self, StarknetApiError> {
        match felt {
            StarkFelt::ZERO => Ok(DataAvailabilityMode::L1),
            StarkFelt::ONE => Ok(DataAvailabilityMode::L2),
            _ => Err(StarknetApiError::OutOfRange {
                string: format!("Invalid data availability mode: {felt}."),
            }),
        }
    }
}
