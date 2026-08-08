use crate::source::{Source, SourceKind};

pub trait Collector {
    type Date;

    type Collection<'a>: IntoIterator<Item = &'a dyn Source<Date = Self::Date>>
    where
        Self: 'a;

    fn sources(&self) -> Self::Collection<'_>;

    #[must_use]
    fn contains(&self, date: &Self::Date, kind: SourceKind) -> bool {
        for src in self.sources() {
            if src.kind() == kind && src.contains(date) {
                return true;
            }
        }

        false
    }
}
