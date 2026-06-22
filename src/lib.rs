#![no_std]


use core::convert::Infallible;


/// Trait for infallible date validations.
///
/// `Source` refines the [`TrySource`] trait for infallible date
/// validations.
pub trait Source : TrySource<Error = Infallible> {
    fn is_valid(&self, date: Self::Date) -> bool;
}


pub trait TrySource {
    type Date;
    type Error: core::error::Error;

    fn try_is_valid(&self, date: Self::Date) -> Result<bool, Self::Error>;
}
