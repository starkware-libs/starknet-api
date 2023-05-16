#[macro_export]
macro_rules! impl_from_small_uints {
    ($struct_type:ty) => {
        $crate::impl_from_small_uints!($struct_type, u8, u16, u32, u64);
    };

    ($struct_type:ty, $($small_uint_type:ty),+) => {
        $(
            impl From<$small_uint_type> for $struct_type {
                fn from(x: $small_uint_type) -> Self {
                    Self::from(u128::from(x))
                }
            }
        )+
    };
}
