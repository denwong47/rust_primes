use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use strum_macros::EnumIter;

use pyo3::prelude::*;

use crate::primes::{
    Sievable,
    SieveOfAtkin,
    SieveOfEratosthenes,
    SieveOfEratosthenesThreaded,
};

#[pyclass(module="rust_primes")]
#[derive(Copy, Clone, Debug, Hash, EnumIter)]
pub enum SieveMethod {
    ATKIN,
    ERATOSTHENES,
    ERATOSTHENES_THREADED,
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
        return Self::ERATOSTHENES;
    }
}
impl Sievable for SieveMethod {
    fn sieve(&self, ubound:u64) ->
    ndarray::ArrayBase<ndarray::OwnedRepr<bool>, ndarray::Dim<[usize; 1]>> {
        return match self {
            Self::ATKIN => SieveOfAtkin::sieve(ubound),
            Self::ERATOSTHENES => SieveOfEratosthenes::sieve(ubound),
            Self::ERATOSTHENES_THREADED => SieveOfEratosthenesThreaded::sieve(ubound),
        }
    }
}
