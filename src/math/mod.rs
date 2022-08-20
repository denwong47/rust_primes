#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]


/// Determines if a number is a prime number.
/// This is a wrapper around `is_prime_with_known_primes`, which actually does the work.
/// The difference is that `is_prime_with_known_primes` requires a list of all known primes to be passed to it, up to and including the `sqrt` of the number itself.
/// `is_prime` will enquire that list using `list_primes` first, before calling `is_prime_with_known_primes`.
pub fn is_prime(
    num:u64,
) -> bool {

    let _list_of_primes:Vec<u64> = list_primes(
        (num as f64).sqrt() as u64, 
    );
    
    return is_prime_with_known_primes(
        num,
        &_list_of_primes,
    )
}

fn is_prime_with_known_primes(
    num:u64,
    primes:&[u64],
) -> bool {

    for &_prime in primes {
        if num % _prime == 0 {
            return false;
        } else if (_prime as f64) >= (num as f64).sqrt() {
            return true;
        }
    }

    return true;
}

pub fn list_primes(
    num:u64,
) -> Vec<u64> {
    let mut _list_of_primes:Vec<u64> = Vec::new();

    for _candidate in 2..num {
        if is_prime_with_known_primes(_candidate, &_list_of_primes) {
            _list_of_primes.push(_candidate)
        }
    }

    return _list_of_primes;
}