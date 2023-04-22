use super::{atomics, types};
use super::{CanBeTrue, OwnedSieve, SievedArray};
use std::sync::atomic::Ordering;

#[allow(unused_imports)]
use crate::py_compatibility::enums::SieveMethod;

/// A trait for [`SieveMethod`] enum as well as its variants, allowing for the unified
/// [`CanSieve::sieve()`] method to be implemented. This method is then
/// used to implement the downstream methods of
/// [`CanSieve::list_primes()`], [`CanSieve::count_primes()`] and
/// [`CanSieve::is_prime()`].
pub trait CanSieve<E>
where
    E: CanBeTrue,
{
    fn sieve(&self, ubound: u64) -> OwnedSieve<E>;

    fn list_primes(&self, ubound: u64, n_limit: Option<u64>) -> Vec<u64> {
        self.sieve(ubound).collect_into_primes(n_limit)
    }

    fn count_primes(&self, ubound: u64) -> u64 {
        self.sieve(ubound).count_primes()
    }

    fn is_prime(&self, num: u64) -> bool {
        let _list_of_primes: Vec<u64> = self.list_primes((num as f64).sqrt() as u64, None);

        for _prime in _list_of_primes {
            if num % _prime == 0 {
                // We found a factor! Get me out of here!
                return false;
            }
        }

        true
    }
}

/// Special trait for [AtomicSieves], which
///
/// [AtomicSieves]: types::AtomicSieve
pub trait IsAtomicSieve {
    fn iter_lowest_primes<'a>(&'a self) -> atomics::IterAtomicSievePrimes<'a>;
    fn to_non_atomic(&self) -> types::NonAtomicSieve;
}

impl IsAtomicSieve for types::AtomicSieve {
    fn iter_lowest_primes<'a>(&'a self) -> atomics::IterAtomicSievePrimes<'a> {
        return atomics::IterAtomicSievePrimes::new(self);
    }

    fn to_non_atomic(&self) -> types::NonAtomicSieve {
        return self.map(|abool| abool.load(Ordering::Relaxed));
    }
}
