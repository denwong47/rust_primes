use std::{cmp};
use std::vec::{Vec};
use ndarray::{Array, ArrayBase, Dim, OwnedRepr, s};


/// Determines if a number is a prime number.
/// This is a wrapper around `is_prime_with_known_primes`, which actually does the work.
/// The difference is that `is_prime_with_known_primes` requires a list of all known primes to be passed to it, up to and including the `sqrt` of the number itself.
/// `is_prime` will enquire that list using `list_primes` first, before calling `is_prime_with_known_primes`.
pub fn is_prime(
    num:u64,
) -> bool {

    // If we don't have a list of primes yet, find a list of primes to evaluate against first.
    let _list_of_primes:Vec<u64> = list_primes(
        (num as f64).sqrt() as u64,
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

/// Returns an `ndarray` of `bool` that indicates whether its index is a prime number.
/// Private function, used for `list_primes` and `count_primes`.
///
/// Using ndarray, the Sieve of Eratosthenes turns out to be the optimal method of calculating
/// primes. This method does not require threading; instead it relies on ndarray's efficiency when
/// using `slice_mut` over steps to achieve incredible speed.
fn prime_mask(ubound:u64) -> ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> {
    let mut sieve = Array::from_elem(((ubound+1) as usize, ), true);

    // Set 0 and 1 to false
    sieve.slice_mut(s![0..cmp::min(2, sieve.len())]).fill(false);

    // Starting from the 2, we sieve through all the prime numbers and mark them as non-primes.
    // Note that we have to make sure that 2 will be checked at the very least, otherwise no primes
    // will be calculated up to prime_mask(7)!
    if ubound > 2 {
        for prime in 2..cmp::max(3, ((ubound+1) as f64).sqrt().ceil() as usize){
            if sieve[prime] {
                let mut factors = sieve.slice_mut(s![prime*2..; prime]);

                factors.fill(false);
            }
        }
    }

    return sieve;
}

/// List all the primes within `ubound`.
/// Calls `prime_mask`, and apply the mask on `enumerate`.
pub fn list_primes(ubound:u64) -> Vec<u64>{
    let sieve = prime_mask(ubound);

    // We gather up everything
    let result: Vec<u64> = sieve.iter()
                                .enumerate()
                                .filter(|&(_, &value)| value)
                                .map(|(index, _)| index as u64)
                                .collect();

    return result;
}

/// Return the number of primes within `ubound`.
/// Calls `prime_mask`, and filter it by `true` before getting its `len`.
pub fn count_primes(ubound:u64) -> u64{
    let sieve = prime_mask(ubound);

    // We gather up everything
    let result: u64 =  sieve.iter()
                            .filter(|&value| *value)
                            .count() as u64;

    return result;
}
