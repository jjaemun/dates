use crate::{source::SourceKind, collector::Collector};



pub struct Calendar<C>
where
    C: Collector,
{
    pub collector: C,
}

impl<C> Calendar<C>
where
    C: Collector,
{
    type Date = C::Date;
    
    pub fn new(collector: C) -> Self {
        Self { collector }
    }
    
    #[must_use]
    #[inline]
    pub fn is_weekend(&self, date: &Self::Date) -> bool {
        self.collector.contains(date, SourceKind::Weekend)
    }

    #[must_use]
    #[inline]
    pub fn is_weekday(&self, date: &Self::Date) -> bool {
        !self.is_weekend(date)
    }

    #[must_use]
    #[inline]
    pub fn is_holiday(&self, date: &Self::Date) -> bool {
        self.collector.contains(date, SourceKind::Holiday)
    }
    
    #[must_use]
    #[inline]
    pub fn is_business_day(&self, date: &Self::Date) -> bool {
        !self.is_weekend(date) && !self.is_holiday(date)
    }
}
    
