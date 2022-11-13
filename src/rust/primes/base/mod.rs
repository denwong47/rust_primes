use std::vec::{Vec};


use crate::py_compatibility::enums;

mod sieves;
pub use sieves::{
    Sieve,
    Sievable,
    SieveOfAtkin,
    SieveOfEratosthenes,
    SieveOfEratosthenesThreaded,
};

/// Determines if a number is a prime number.
/// This is a wrapper around `is_prime_with_known_primes`, which actually does the work.
/// The difference is that `is_prime_with_known_primes` requires a list of all known primes to be passed to it, up to and including the `sqrt` of the number itself.
/// `is_prime` will enquire that list using `list_primes` first, before calling `is_prime_with_known_primes`.
pub fn is_prime(
    sieve:enums::SieveMethod,
    num:u64,
) -> bool {

    // If we don't have a list of primes yet, find a list of primes to evaluate against first.
    let _list_of_primes:Vec<u64> = list_primes(
        sieve,
        (num as f64).sqrt() as u64,
        None,
    );

    return is_prime_with_known_primes(
        num,
        &_list_of_primes,
    )
}

/// Private function
/// This actually do the work of checking primes.
/// Mandatory: Give it a slice of SORTED prime numbers up to at least sqrt of num.
fn is_prime_with_known_primes(
    num:u64,
    primes:&[u64],
) -> bool {

    for &_prime in primes {
        if num % _prime == 0 {
            // We found a factor! Get me out of here!
            return false;
        } else if (_prime as f64) >= (num as f64).sqrt() {
            // Assuming the slice is sorted, by the time that num.sqrt() is found not to be a prime,
            // there will be no possible primes beyond this number. We can safely stop here and assume its a prime.
            return true;
        }
    }

    return true;
}


/// List all the primes within `ubound`.
/// Calls `prime_mask`, and apply the mask on `enumerate`.
pub fn list_primes(
    sieve:enums::SieveMethod,
    ubound:u64,
    n_limit:Option<u64>,
) -> Vec<u64>{
    let sieve:Sieve = sieve.sieve(ubound);

    // We gather up everything
    return sieves::collect(
        &sieve, n_limit
    )
}

/// Return the number of primes within `ubound`.
/// Calls `prime_mask`, and filter it by `true` before getting its `len`.
pub fn count_primes(
    sieve:enums::SieveMethod,
    ubound:u64,
) -> u64{
    let sieve:Sieve = sieve.sieve(ubound);

    // We gather up everything
    return sieves::count(&sieve);
}
