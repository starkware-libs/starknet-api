use crate::hash::StarkFelt;

pub(crate) fn hash_array_preimage(array: &Vec<StarkFelt>) -> Vec<StarkFelt> {
    [vec![array.len().into()], array.iter().copied().collect::<Vec<StarkFelt>>()].concat()
}
