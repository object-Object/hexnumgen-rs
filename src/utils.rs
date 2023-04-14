use itertools::Itertools;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::Signed;
use parking_lot::{Condvar, Mutex, RwLockReadGuard};
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

impl<T> From<Ratio<T>> for NonZeroSign
where
    T: Clone + Integer + Signed,
{
    fn from(value: Ratio<T>) -> Self {
        if value.is_negative() {
            NonZeroSign::Negative
        } else {
            NonZeroSign::Positive
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

// adapted from https://stackoverflow.com/a/67128189
pub fn drain_every_other<T>(items: &mut Vec<T>) -> Vec<T> {
    let mut opt_items = items.drain(..).map(Some).collect_vec();

    let picked = opt_items.iter_mut().step_by(2).map(|i| i.take().unwrap()).collect_vec();

    items.extend(opt_items.into_iter().flatten());

    picked
}

// from https://github.com/Amanieu/parking_lot/issues/165
#[derive(Default)]
pub struct CondvarAny {
    pub c: Condvar,
    m: Mutex<()>,
}

impl CondvarAny {
    pub fn wait<T>(&self, g: &mut RwLockReadGuard<'_, T>) {
        let guard = self.m.lock();
        RwLockReadGuard::unlocked(g, || {
            // Move the guard in so it gets unlocked before we re-lock g
            let mut guard = guard;
            self.c.wait(&mut guard);
        });
    }
}
