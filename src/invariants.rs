
pub trait Searchable<T> 
where
    T: Ord,
{
    fn contains(&self, what: &T) -> bool;
}
