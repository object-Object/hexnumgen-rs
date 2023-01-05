use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
pub enum NonZeroSign {
    Positive,
    Negative,
}

impl From<i32> for NonZeroSign {
    fn from(num: i32) -> Self {
        if num >= 0 {
            NonZeroSign::Positive
        } else {
            NonZeroSign::Negative
        }
    }
}

pub fn cloned_push<T>(vec: &[T], item: T) -> Vec<T>
where
    T: Clone,
{
    vec.iter().chain(&[item]).cloned().collect()
}

pub fn cloned_union<T, U>(hashset: &HashSet<T>, iter: U) -> HashSet<T>
where
    T: Eq + Hash + Clone,
    U: IntoIterator<Item = T>,
{
    hashset.union(&HashSet::from_iter(iter)).cloned().collect()
}

pub fn cloned_union_single<T>(hashset: &HashSet<T>, item: T) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    cloned_union(hashset, [item])
}
