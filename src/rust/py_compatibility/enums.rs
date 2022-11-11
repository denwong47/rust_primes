use pyo3::prelude::*;

use crate::primes::{
    Sievable,
    SieveOfAtkin,
    SieveOfEratosthenes,
};

#[pyclass(module="rust_primes")]
#[derive(Copy, Clone, Debug)]
pub enum SieveMethod {
    ATKIN,
    ERATOSTHENES
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
            Self::ERATOSTHENES => SieveOfEratosthenes::sieve(ubound)
        }
    }
}
