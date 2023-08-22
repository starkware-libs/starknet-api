use serde::{Deserialize, Serialize};

use crate::hash::StarkFelt;
use crate::StarknetApiError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DataAvailabilityMode {
    L1(u32),
    L2(u32),
}

impl TryFrom<StarkFelt> for DataAvailabilityMode {
    type Error = StarknetApiError;

    fn try_from(felt: StarkFelt) -> Result<Self, StarknetApiError> {
        match felt {
            StarkFelt::ZERO => Ok(DataAvailabilityMode::L1(0_u32)),
            StarkFelt::ONE => Ok(DataAvailabilityMode::L2(0_u32)),
            _ => Err(StarknetApiError::OutOfRange {
                string: format!("Invalid data availability mode: {felt}."),
            }),
        }
    }
}
