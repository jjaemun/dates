#![no_std]

use core::convert::Infallible;

pub trait Source: TrySource<Error = Infallible> {
    fn contains(&self, date: &Self::Date) -> bool;
}

impl<S> Source for S
where 
    S: TrySource<Error = Infallible>,
{}  

pub trait TrySource {
    type Error: core::error::Error;
    type Date;

    fn try_contains(&self, date: &Self::Date) -> Result<bool, Self::Error>;
}
