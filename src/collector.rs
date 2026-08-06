use core::convert::Infallible;
use source::{SourceKind, TrySource};

pub trait Collector: TryCollector<Error = Infallible> {
    fn contains(&self, date: &Self::Date, kind: SourceKind) -> bool;
}

impl<C> Collector for C
where
    C: TryCollector<Error = Infallible>,
{
    #[inline]
    fn contains(&self, date: &Self::Date, kind: SourceKind) -> bool {
        match self.try_contains(date, kind) {
            Ok(contained) => contained,
        }
    }
}

pub trait TryCollector {
    type Error: core::error::Error;
    type Date;

    type Collection<'a>: IntoIterator<
        Item = &'a dyn TrySource<Error = Self::Error, Date = Self::Date>,
    >
    where
        Self: 'a;

    fn sources(&self) -> Self::Collection<'_>;

    #[inline]
    fn try_contains(&self, date: &Self::Date, kind: SourceKind) -> Result<bool, Self::Error> {
        for src in self.sources() {
            if src.kind() == kind && src.try_contains(date)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
