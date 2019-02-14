pub trait OverrideWith {
    fn override_with(&mut self, _: Self);
}

impl<T> OverrideWith for Option<T> {
    fn override_with(&mut self, other: Self) {
        if other.is_some() {
            *self = other
        }
    }
}

macro_rules! impl_override_with_for {
    ($type:path) => {
        impl OverrideWith for $type {
            fn override_with(&mut self, other: Self) {
                *self = other;
            }
        }
    };
}

impl_override_with_for!(bool);
impl_override_with_for!(char);
impl_override_with_for!(u8);
impl_override_with_for!(u16);
impl_override_with_for!(u32);
impl_override_with_for!(u64);
impl_override_with_for!(u128);
impl_override_with_for!(i8);
impl_override_with_for!(i16);
impl_override_with_for!(i32);
impl_override_with_for!(i64);
impl_override_with_for!(i128);
impl_override_with_for!(usize);
impl_override_with_for!(String);
// TODO Add more?

impl<T> OverrideWith for Vec<T> {
    fn override_with(&mut self, other: Self) {
        *self = other;
    }
}
