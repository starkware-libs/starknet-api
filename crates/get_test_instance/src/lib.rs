#[cfg(any(feature = "testing", test))]
pub trait GetTestInstance: Sized {
    fn get_test_instance() -> Self;
}
