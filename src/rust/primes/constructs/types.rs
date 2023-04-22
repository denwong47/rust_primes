use ndarray::{ArrayBase, Dim, OwnedRepr, ViewRepr};
use std::sync::atomic::AtomicBool;

pub type Sieve<T> = ArrayBase<T, Dim<[usize; 1]>>;

#[allow(dead_code)]
pub type OwnedSieve<E> = Sieve<OwnedRepr<E>>;

#[allow(dead_code)]
pub type ViewSieve<E> = Sieve<ViewRepr<E>>;

pub type NonAtomicSieve = OwnedSieve<bool>;
pub type AtomicSieve = OwnedSieve<AtomicBool>;
