use serde::{Deserialize, Serialize};

use crate::hash::StarkFelt;
use crate::OutOfRangeError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DataAvailabilityMode {
    L1 = 0,
    L2 = 1,
}

impl TryFrom<StarkFelt> for DataAvailabilityMode {
    type Error = OutOfRangeError;

    fn try_from(felt: StarkFelt) -> Result<Self, OutOfRangeError> {
        match felt {
            StarkFelt::ZERO => Ok(DataAvailabilityMode::L1),
            StarkFelt::ONE => Ok(DataAvailabilityMode::L2),
            _ => {
                Err(OutOfRangeError { string: format!("Invalid data availability mode: {felt}.") })
            }
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
