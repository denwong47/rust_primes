//!
//! # Not too terrible Prime Calculator
//! ### written in Rust with Python bindings
//!
//! See [Python docs] for use in Python.
//!
//! Sample output and runtime on an Apple M1 Pro:
//!
//! ```text
//! Calculating all primes under 10^4:
//! - SieveMethod.ATKIN                       : 0.000024s
//! - SieveMethod.ERATOSTHENES                : 0.000011s
//! - SieveMethod.ERATOSTHENES_THREADED       : 0.000142s
//! - SieveMethod.ERATOSTHENES_ATOMIC         : 0.000115s
//!
//! Calculating all primes under 10^5:
//! - SieveMethod.ATKIN                       : 0.000190s
//! - SieveMethod.ERATOSTHENES                : 0.000099s
//! - SieveMethod.ERATOSTHENES_THREADED       : 0.000221s
//! - SieveMethod.ERATOSTHENES_ATOMIC         : 0.000288s
//!
//! Calculating all primes under 10^6:
//! - SieveMethod.ATKIN                       : 0.001648s
//! - SieveMethod.ERATOSTHENES                : 0.001109s
//! - SieveMethod.ERATOSTHENES_THREADED       : 0.001364s
//! - SieveMethod.ERATOSTHENES_ATOMIC         : 0.001368s
//!
//! Calculating all primes under 10^7:
//! - SieveMethod.ATKIN                       : 0.014422s
//! - SieveMethod.ERATOSTHENES                : 0.011521s
//! - SieveMethod.ERATOSTHENES_THREADED       : 0.024931s
//! - SieveMethod.ERATOSTHENES_ATOMIC         : 0.011437s
//!
//! Calculating all primes under 10^8:
//! - SieveMethod.ATKIN                       : 0.447571s
//! - SieveMethod.ERATOSTHENES                : 0.423881s
//! - SieveMethod.ERATOSTHENES_THREADED       : 0.222338s
//! - SieveMethod.ERATOSTHENES_ATOMIC         : 0.153570s
//!
//! Calculating all primes under 10^9:
//! - SieveMethod.ATKIN                       : 5.407987s
//! - SieveMethod.ERATOSTHENES                : 5.830421s
//! - SieveMethod.ERATOSTHENES_THREADED       : 2.663690s
//! - SieveMethod.ERATOSTHENES_ATOMIC         : 2.096283s
//!
//! ```
//!
//! [Python docs]: ../../../index.html
use pyo3::prelude::*;

mod primes;
mod py_compatibility;
use py_compatibility::enums;

pub use primes::constructs::*;

/// A module containing all the exports of this library.
pub mod prelude {
    pub use crate::primes::constructs::*;
    pub use crate::primes::*;
    pub use crate::py_compatibility::enums::SieveMethod;
}

/// Rust library function.
///
/// Determines if a number is a prime number.
/// This is a wrapper around `is_prime_with_known_primes`, which actually does the work.
/// The difference is that `is_prime_with_known_primes` requires a list of all known primes to be passed to it, up to and including the `sqrt` of the number itself.
/// `is_prime` will enquire that list using `list_primes` first, before calling `is_prime_with_known_primes`.
#[pyfunction(num, "*", method)]
fn is_prime(num: u64, method: Option<&enums::SieveMethod>) -> PyResult<bool> {
    Ok(method
        .unwrap_or(&enums::SieveMethod::default())
        .is_prime(num))
}

/// Rust library function.
///
/// List all the primes within `num`.
/// Calls `prime_mask`, and apply the mask on `enumerate`.
#[pyfunction(num, "*", method)]
fn list_primes(num: u64, method: Option<&enums::SieveMethod>) -> PyResult<Vec<u64>> {
    Ok(method
        .unwrap_or(&enums::SieveMethod::default())
        .list_primes(num, None))
}

/// Rust library function.
///
/// Return the number of primes within `num`.
/// Calls `prime_mask`, and filter it by `true` before getting its `len`.
#[pyfunction(num, "*", method)]
fn count_primes(num: u64, method: Option<&enums::SieveMethod>) -> PyResult<u64> {
    Ok(method
        .unwrap_or(&enums::SieveMethod::default())
        .count_primes(num))
}

/// Rust library function.
///
/// Return the upper bound of the nth prime.
#[pyfunction]
fn upper_bound_of_nth_prime(n: u64) -> PyResult<u64> {
    Ok(primes::upper_bound_of_nth_prime(n))
}

/// Rust library function.
///
/// List the first `n` primes.
#[pyfunction(num, "*", method)]
fn list_n_primes(n: u64, method: Option<&enums::SieveMethod>) -> PyResult<Vec<u64>> {
    Ok(primes::list_n_primes(
        *method.unwrap_or(&enums::SieveMethod::default()),
        n,
    ))
}

/// Rust library function.
///
/// List the first `n` primes.
#[pyfunction(num, "*", method)]
fn nth_prime(n: u64, method: Option<&enums::SieveMethod>) -> PyResult<Option<u64>> {
    Ok(primes::nth_prime(
        *method.unwrap_or(&enums::SieveMethod::default()),
        n,
    ))
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_rust_primes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    m.add_function(wrap_pyfunction!(list_primes, m)?)?;
    m.add_function(wrap_pyfunction!(count_primes, m)?)?;
    m.add_function(wrap_pyfunction!(upper_bound_of_nth_prime, m)?)?;
    m.add_function(wrap_pyfunction!(list_n_primes, m)?)?;
    m.add_function(wrap_pyfunction!(nth_prime, m)?)?;

    m.add_class::<py_compatibility::enums::SieveMethod>()?;

    Ok(())
}
