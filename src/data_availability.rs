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

impl From<DataAvailabilityMode> for StarkFelt {
    fn from(data_availability_mode: DataAvailabilityMode) -> StarkFelt {
        match data_availability_mode {
            DataAvailabilityMode::L1 => StarkFelt::ZERO,
            DataAvailabilityMode::L2 => StarkFelt::ONE,
        }
    }
}

#[derive(
    Clone, Default, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum L1DataAvailabilityMode {
    #[default]
    Calldata,
    Blob,
}
