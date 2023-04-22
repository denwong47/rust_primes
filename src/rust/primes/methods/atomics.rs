use ndarray::{s, Array};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::primes::constructs::*;

/// Atomic SieveOfEratosthenes supporting threaded operations.
pub struct SieveOfEratosthenesAtomic;
impl SieveOfEratosthenesAtomic {
    pub fn new() -> Self {
        Self {}
    }
}
impl CanSieve<AtomicBool> for SieveOfEratosthenesAtomic {
    fn sieve(&self, ubound: u64) -> AtomicSieve {
        let sieve: AtomicSieve =
            Array::from_shape_fn(((ubound + 1) as usize,), |i| AtomicBool::new(i > 1));

        sieve
            .iter_lowest_primes()
            .par_bridge()
            .into_par_iter()
            .for_each(|prime| {
                let slice = sieve.slice(s![prime*2..; prime]);

                slice.iter().for_each(|abool| {
                    abool.store(false, Ordering::Relaxed);
                })
            });

        sieve
    }
}
