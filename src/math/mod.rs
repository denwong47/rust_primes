#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::thread;
use std::cmp;
use std::sync;

use num_format::{Locale, ToFormattedString};

const MAX_WORKERS: usize = 10;

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

/// Private function 
fn is_prime_with_known_primes(
    num:u64,
    primes:&[u64],
) -> bool {

    // println!("Evaluating {} using {:?}...", num, primes);

    for &_prime in primes {
        if num % _prime == 0 {
            return false;
        } else if (_prime as f64) >= (num as f64).sqrt() {
            return true;
        }
    }

    return true;
}

/// ### List all the prime up to `num` by going through all of them one by one.
/// 
/// There is no threading in this function - it simply loop through the range from 2 to `num`.
pub fn list_primes_unthreaded(
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

/// ### List all the prime up to `num` by going through all of them one by one.
/// 
/// There is no threading in this function - it simply loop through the range from 2 to `num`.
pub fn list_primes_threaded(
    num:u64,
) -> Vec<u64> {
    let mut _list_of_primes:Vec<u64>                = vec![2];

    let mut _range_max:u64              = _list_of_primes[0];
    
    while _range_max < num {
        let _range_min:u64 = _range_max +1;
        _range_max = cmp::min(num+1, _range_min*_range_min);

        let mut _threads:Vec<thread::JoinHandle<Option<u64>>> = Vec::with_capacity(MAX_WORKERS);

        {
            let mut _list_of_primes_in_range:Vec<u64> = Vec::new();
            let _arc_of_primes:sync::Arc<Vec<u64>> = sync::Arc::new(_list_of_primes.clone());
            println!("Cloned {} items to an Arc.", _list_of_primes.len());

            for _num in _range_min.._range_max {
                let _num_in_scope:u64 = _num;
                let _list_of_primes_ref = sync::Arc::clone(&_arc_of_primes);

                if _num % 10000 == 0{
                    println!("Spawning thread for {}...", _num)
                }
    
                _threads.push(
                    thread::spawn(move ||->Option<u64> {
                        if is_prime_with_known_primes(_num_in_scope, &_list_of_primes_ref){
                            return Some(_num_in_scope)
                        } else {
                            return None
                        }
                    })
                );

                while 
                    _threads.len() >= MAX_WORKERS || 
                    (
                        _num>=_range_max-1 &&
                        _threads.len()>0
                    )
                
                {
                    // Wait til the first thread is done first
                    let _result:Option<u64> = _threads.pop().expect("No Threads left!").join().unwrap();
                    match _result {
                        Some(num) => _list_of_primes_in_range.push(num),
                        None => (),
                    }
                }
            }

            _list_of_primes.append(&mut _list_of_primes_in_range);
        }

        for _thread in _threads{
            let _result:Option<u64> = _thread.join().unwrap();

            match _result {
                Some(num) => _list_of_primes.push(num),
                None => (),
            }
        }

        _list_of_primes.sort();

        println!("{} primes found up to {}.", _list_of_primes.len().to_formatted_string(&Locale::en), _range_max.to_formatted_string(&Locale::en))
    }
        
    
    return _list_of_primes;
}

/// Alias Function
pub fn list_primes(
    num:u64,
) -> Vec<u64> {
    return list_primes_threaded(num);
}