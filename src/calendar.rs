use crate::{Source, TrySource};

pub struct Calendar {}

pub trait Adjust {
    type Date;

    fn adjust(&self, date: Self::Date, conv: BusinessDay) -> Self::Date;
}

pub trait Classify {
    type Date;

    fn is_weekday(&self, date: Self::Date) -> bool {
        !self.is_weekend(date)
    }

    fn is_weekend(&self, date: Self::Date) -> bool;

    fn is_business_day(&self, date: Self::Date) -> bool {
        !self.is_holiday(date) && !self.is_weekend(date)
    }

    fn is_holiday(&self, date: Self::Date) -> bool;
} 

pub trait Shift {
    type Date;
    type Offset;

    fn advance(&self, date: Self::Date, offset: Self::Offset) -> Self::Date;

    fn reverse(&self, date: Self::Date, offset: Self::Offset) -> Self::Date;
}
