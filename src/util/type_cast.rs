/// A trait to allow generic code to cast between types.
pub(crate) trait TypeCast<T> {
    /// Cast the value to the target type `T`.
    fn cast(&self) -> T;
}

/// Generic implementation for delegating the cast of a reference to the cast of
/// the value it points to.
impl<T, U> TypeCast<U> for &T
where
    T: TypeCast<U>,
{
    fn cast(&self) -> U {
        (**self).cast()
    }
}

impl TypeCast<f64> for f64 {
    fn cast(&self) -> f64 {
        *self
    }
}

impl TypeCast<f64> for u64 {
    fn cast(&self) -> f64 {
        *self as f64
    }
}
