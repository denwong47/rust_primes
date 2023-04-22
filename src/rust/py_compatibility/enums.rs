use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use strum_macros::EnumIter;

use pyo3::prelude::*;

use crate::primes::constructs::OwnedSieve;
use crate::primes::{
    constructs::{CanSieve, IsAtomicSieve},
    SieveOfAtkin, SieveOfEratosthenes, SieveOfEratosthenesAtomic, SieveOfEratosthenesThreaded,
};

#[allow(unused_imports)]
use crate::{count_primes, is_prime, list_n_primes, list_primes};

/// Methods of sieving.
///
/// To be used with :func:`~rust_primes.count_primes`
#[pyclass(module = "rust_primes")]
#[derive(Copy, Clone, Debug, Hash, EnumIter)]
#[allow(non_camel_case_types)]
pub enum SieveMethod {
    // Follows Python casing conventions.
    /// Modern method, but less well optimised by the compiler;
    /// not necessarily more performant.
    ATKIN,

    /// The ancient method. Using the
    /// :meth:`ndarray.slice_mut().step` method, the compiler can optimise the inner loop
    /// to a close to ``O(n)`` operation.
    ERATOSTHENES,

    /// Same method as :attr:`ERATOSTHENES`, but
    /// conducted on an array of Atomic booleans instead. Allows threading to run
    /// withoutmuch overhead. *This is the default.*
    ERATOSTHENES_ATOMIC,

    /// *Experimental*. An attempt to introduce
    /// threading into :attr:``ERATOSTHENES``. It works by using :attr:`ERATOSTHENES` to
    /// create a base array of primes, upto :func:`Math.sqrt` of the upper bound, then
    /// split the list of primes into equal size for each worker, then each worker sieve
    /// the rest of the (larger) numbers.
    ERATOSTHENES_THREADED,
    // WHEEL_FACTORISED_PRIME_CHECK,
}
#[pymethods]
impl SieveMethod {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
impl Default for SieveMethod {
    fn default() -> Self {
        return Self::ERATOSTHENES_ATOMIC;
    }
}
impl CanSieve<bool> for SieveMethod {
    fn sieve(&self, ubound: u64) -> OwnedSieve<bool> {
        match self {
            Self::ATKIN => SieveOfAtkin::new().sieve(ubound),
            Self::ERATOSTHENES => SieveOfEratosthenes::new().sieve(ubound),
            Self::ERATOSTHENES_ATOMIC => SieveOfEratosthenesAtomic::new()
                .sieve(ubound)
                .to_non_atomic(),
            Self::ERATOSTHENES_THREADED => SieveOfEratosthenesThreaded::new().sieve(ubound),
        }
    }
}
