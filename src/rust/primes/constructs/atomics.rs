use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use super::types::AtomicSieve;

#[allow(unused_imports)]
use super::can_sieve::IsAtomicSieve;

/// An [`Iterator`] struct to yield the lowest prime currently found in the sieve,
/// up to the square root of the length of the sieve.
///
/// This struct is typically created from [`IsAtomicSieve::iter_lowest_primes()`]. It
/// stores an immutable pointer to the [`AtomicSieve`] it was created from at the time
/// of instantiation, hence it cannot outlive the [`AtomicSieve`] it is created for.
pub struct IterAtomicSievePrimes<'a> {
    counter: Arc<Mutex<usize>>,
    atomic_sieve: &'a AtomicSieve,
}
impl<'a> IterAtomicSievePrimes<'a> {
    pub fn new(sieve: &'a AtomicSieve) -> Self {
        Self {
            counter: Arc::new(Mutex::new(2)),
            atomic_sieve: sieve,
        }
    }
}
impl<'a> Iterator for IterAtomicSievePrimes<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut _current = self.counter.lock().unwrap();

        for candidate in *_current..(self.atomic_sieve.len() as f64).sqrt().ceil() as usize {
            if self
                .atomic_sieve
                .get(candidate)
                .and_then(|abool| {
                    if abool.load(Ordering::Relaxed) {
                        Some(true)
                    } else {
                        None
                    }
                })
                .is_some()
            {
                *_current = candidate + 1;
                return Some(candidate);
            }
        }

        None
    }
}
