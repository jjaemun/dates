use source::{TrySource, SourceKind};


pub trait Collector {
    type Error: core::error::Error;
    type Date;

    fn sources(&self) -> 
        &[Box<dyn TrySource<Error = Self::Error, Date = Self::Date>>];

    #[inline]
    fn contains(&self, date: &Self::Date, 
                kind: SourceKind) -> Result<bool, Self::Error> {
        let kindlike = self.sources()
                            .into_iter().filter(move |src| src.kind() == kind);

        for src in kindlike {
            if src.try_contains(date)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
