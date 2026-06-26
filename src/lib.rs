#![no_std]

use core::convert::Infallible;

use crate::invariants::Searchable;

pub trait Source: TrySource<Error = Infallible> {
    fn is_valid(&self, date: Self::Date) -> bool;

    fn contains<S>(&self, date: &Self::Date, searchable: &S) -> bool
    where 
        S: Searchable<Self::Date>, 
    {
        searchable.contains(date)
    }
}

pub trait TrySource {
    type Error: core::error::Error;
    type Date;

    fn try_is_valid(&self, date: Self::Date) -> Result<bool, Self::Error>;

    fn try_contains<S>(&self, date: &Self::Date, 
                       searchable: &S) -> Result<bool, Self::Error>
    where
        S: Searchable<Self::Date>,
    {
        Ok(searchable.contains(date))
    }
}
