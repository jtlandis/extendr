use super::{Error, Result, Robj};

pub trait FromRust<T>
where
    Robj: From<T>,
{
    fn from_rust(from: T) -> Robj {
        Robj::from(from)
    }
}

impl<T> FromRust<T> for Robj where Robj: From<T> {}

/// TryFrom<Robj> for T
pub trait TryFromR<R>
where
    Self: Sized,
{
    fn try_from_r(value: R) -> Result<Self>;
}

impl<T> TryFromR<Robj> for T
where
    T: TryFrom<Robj, Error = Error>,
{
    fn try_from_r(value: Robj) -> Result<T> {
        T::try_from(value)
    }
}

impl<'a, T> TryFromR<&'a Robj> for T
where
    T: TryFrom<&'a Robj, Error = Error>,
{
    fn try_from_r(value: &'a Robj) -> Result<T> {
        T::try_from(value)
    }
}

impl<'a, T> TryFromR<&'a mut Robj> for T
where
    T: TryFrom<&'a mut Robj, Error = Error>,
{
    fn try_from_r(value: &'a mut Robj) -> Result<T> {
        T::try_from(value)
    }
}

pub trait TryIntoRust<T>
where
    Self: Sized,
    T: TryFromR<Self>,
{
    fn try_into_rust(self) -> Result<T> {
        T::try_from_r(self)
    }
}

impl<T> TryIntoRust<T> for Robj where T: TryFrom<Robj, Error = Error> {}
