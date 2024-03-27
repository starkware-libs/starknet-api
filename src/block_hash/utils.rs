use crate::hash::StarkFelt;

pub(crate) fn hash_array_preimage(array: &[StarkFelt]) -> Vec<StarkFelt> {
    [vec![array.len().into()], array.to_vec()].concat()
}
