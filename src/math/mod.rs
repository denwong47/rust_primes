#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]


/// Determines 
/// Something
pub fn is_prime(
    num:u64,
    primes:Option<&[u64]>,
) -> bool {

    // Optional Parameter
    let _primes_to_test:&[u64];

    let _obj:&[u64] = &(2..30).collect::<Vec<u64>>();
    
    match primes {
        Some(p) => _primes_to_test = p,
        None => _primes_to_test = _obj,
    }

    for &_prime in _primes_to_test {
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
        if is_prime(_candidate, Some(&_list_of_primes)) {
            _list_of_primes.push(_candidate)
        }
    }

    return _list_of_primes;
}