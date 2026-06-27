use crate::{Source, TrySource};

pub struct Calendar {}

pub trait Adjust {
    type Date;

    fn adjust(&self, date: Self::Date, conv: BusinessDay) -> Self::Date;
}

pub trait Shift {
    type Date;
    type Offset;
  
    fn advance(&self, date: Self::Date, offset: Self::Offset) -> Self::Date;

    fn reverse(&self, date: Self::Date, offset: Self::Offset) -> Self::Date;
}
