use thiserror::Error;

use crate::hash::StarkFelt;

#[derive(Clone, Copy, Debug)]
pub enum DataAvailabilityMode {
    L1 = 0,
    L2 = 1,
}

#[derive(Debug, Error)]
pub enum DataAvailabilityError {
    #[error("Invalid data availability mode: {data_availability_mode}.")]
    InvalidDataAvailabilityMode { data_availability_mode: StarkFelt },
}

impl TryFrom<StarkFelt> for DataAvailabilityMode {
    type Error = DataAvailabilityError;

    fn try_from(felt: StarkFelt) -> Result<Self, Self::Error> {
        if felt == StarkFelt::from(0_u8) {
            Ok(DataAvailabilityMode::L1)
        } else if felt == StarkFelt::from(1_u8) {
            Ok(DataAvailabilityMode::L2)
        } else {
            Err(DataAvailabilityError::InvalidDataAvailabilityMode { data_availability_mode: felt })
        }
    }
}
