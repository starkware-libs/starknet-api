use super::calculate_root;
use crate::hash::{PoseidonHashCalculator, StarkFelt};

#[test]
fn test_patricia_regression() {
    let root = calculate_root(
        vec![StarkFelt::from(1_u8), StarkFelt::from(2_u8), StarkFelt::from(3_u8)],
        &PoseidonHashCalculator,
    );
    let expected_root =
        StarkFelt::try_from("0x3b5cc7f1292eb3847c3f902d048a7e5dc7702d1c191ccd17c2d33f797e6fc32")
            .unwrap();
    assert_eq!(root, expected_root);
}

#[test]
fn test_edge_patricia_regression() {
    let root = calculate_root(vec![StarkFelt::from(1_u8)], &PoseidonHashCalculator);
    let expected_root =
        StarkFelt::try_from("0x7752582c54a42fe0fa35c40f07293bb7d8efe90e21d8d2c06a7db52d7d9b7e1")
            .unwrap();
    assert_eq!(root, expected_root);
}

#[test]
fn test_binary_patricia_regression() {
    let root =
        calculate_root(vec![StarkFelt::from(1_u8), StarkFelt::from(2_u8)], &PoseidonHashCalculator);
    let expected_root =
        StarkFelt::try_from("0x1c1ba983ee0a0de87d87d67ea3cbee7023aa65f6b7bcf71259f122ea3af80bf")
            .unwrap();
    assert_eq!(root, expected_root);
}
