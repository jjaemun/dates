use core::convert::Infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Weekend,
    Holiday,
}

pub trait Source: TrySource<Error = Infallible> {
    fn contains(&self, date: &Self::Date) -> bool;
}

impl<S> Source for S
where
    S: TrySource<Error = Infallible>,
{
    fn contains(&self, date: &Self::Date) -> bool {
        match self.try_contains(date) {
            Ok(contained) => contained,
        }
    }
}

pub trait TrySource {
    type Error: core::error::Error;
    type Date;

    fn kind(&self) -> SourceKind;

    fn try_contains(&self, date: &Self::Date) -> Result<bool, Self::Error>;
}
