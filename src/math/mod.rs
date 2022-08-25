#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]


///
/// # Not too terrible Prime Calculator in Rust
/// ### by denwong47
/// 
/// Sample output and runtime on an Apple M1 Pro:
/// Within the first 1,000,000 numbers, there are 78,498 prime numbers.
/// Calculating all primes up to 1,000,000 had taken 0.0026658999999999997s.
/// Validating all primes up to 1,000,000 had taken 0.015052s.
/// 
/// Within the first 100,000,000 numbers, there are 5,761,455 prime numbers.
/// Calculating all primes up to 100,000,000 had taken 0.6158214s.
/// Validating all primes up to 100,000,000 had taken 4.697561s.
/// 
/// NOTE:
/// Calculations are done threaded;
/// Validation is done serially, unthreaded.

use std::thread;
use std::cmp;
use std::sync;
use core::ops::Range;

// Thousand Number formatter #9,001
// use num_format::{Locale, ToFormattedString};

// Maximum number of concurrent threads.
// This is important because Rust implements system threads, not green threads.
// So if we spawn more threads than we are supposed to, we overwhelm the OS Scheduler and we slow down.
const MAX_WORKERS:   usize = 40;
const MIN_CHUNKSIZE: usize = 10102;

const UNTHREADED_CUT_OFF: u64 = 2048;

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

    // DEBUG PRINT
    // println!("Evaluating {} using {:?}...", num, primes);

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

/// ### List all the primes within a range 
/// 
/// Internal function, allowing to list primes only within a range.
/// Mandatory: Give it a slice of SORTED prime numbers up to at least sqrt of range::max.
fn list_primes_in_range_unthreaded(
    range:Range<u64>,
    primes:&[u64],
) -> Vec<u64> {
    let mut _list_of_primes:Vec<u64> = Vec::new();

    // We just dumb check everything from 2 up to num.
    for _candidate in range {
        if is_prime_with_known_primes(_candidate, primes) {
            _list_of_primes.push(_candidate)
        }
    }

    return _list_of_primes;
}

/// ### List all the prime up to `num` by going through all of them one by one.
/// 
/// There is no threading in this function - it simply loop through the range from 2 to `num`.
/// Mysteriously... this is very fast. WHAT?
pub fn list_primes_unthreaded(
    num:u64,
) -> Vec<u64> {
    let mut _list_of_primes:Vec<u64> = Vec::new();

    // We just dumb check everything from 2 up to num.
    for _candidate in 2..num+1 {
        if is_prime_with_known_primes(_candidate, &_list_of_primes) {
            _list_of_primes.push(_candidate)
        }
    }

    return _list_of_primes;
}

/// ### List all the prime including and up to num.
/// 
/// How it works:
/// 
/// For any given integer n, the smallest number which has a minimum factor >n will be (n+1)*(n+1).
/// Therefore assuming we know all the primes up to value n: [2, 3, 5... k] where k <= n, we can safely assert that all non-prime numbers up to (n+1)*(n+1)-1 will have at least one factor within our list.
/// 
/// Thus, for every n, we will take what we already know are primes: [2, 3, 5... k] and check for factors in all values between n+1 to (n+1)*(n+1)-1. Now that we know all the primes up to (n+1)*(n+1) (which we know for sure is NOT a prime), then we can take (n+1)*(n+1) as the new n, then repeat.
/// 
/// i.e.
/// n = 2           We check all numbers between 3 to 3*3-1=8 with our prime list of [2],
/// yielding: [2, 3, 5, 7]
/// n = 9           We check all numbers between 10 to 10*10-1=99 with our prime list of [2, 3, 5, 7],
/// yielding: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
/// n = 100         We check all numbers between 101 to 101*101-1=10202 with our prime list ...
/// 
/// We then stop when upper_limit is reached.
/// 
/// On a i7-8700 (65W):
/// PYTHON:
/// Calculating all primes up to 100,000 took 0.062705s.
/// 
/// RUST:
/// Calculating all primes up to 100,000 took 4.592612s.
/// Validating all primes (without threading) up to 100,000 had taken 0.031746s.
/// 
/// ^^ What the!???!!!
/// 

pub fn list_primes_threaded(
    num:u64,
) -> Vec<u64> {

    // We did some experiment and we have found that up to around 3000, unthreaded is faster because of how the early buckets are so small.
    // So this is improved to make the first bucket 2..2048, then we scale from there onwards, presumably threaded.
    let mut _range_max:u64              = cmp::min(UNTHREADED_CUT_OFF, num);
    let mut _list_of_primes:Vec<u64>    = list_primes_unthreaded(_range_max); //vec![2];
    
    while _range_max < num {
        let _range_min:u64 = _range_max +1;
        _range_max = cmp::min(num+1, _range_min*_range_min);

        let _range_count:u64    = _range_max - _range_min + 1;
        let _no_of_chunks:u64   = cmp::min(
            ((_range_count as f64)/(MIN_CHUNKSIZE as f64)).ceil() as u64,
            MAX_WORKERS as u64,
        );
        let _chunk_size:u64     = ((_range_count as f64)/(_no_of_chunks as f64)).ceil() as u64;

        // println!(
        //     "Range from {} to {} has {} members, proposing {} chunks of size {}.",
        //     &_range_min,
        //     &_range_max,
        //     &_range_count,
        //     &_no_of_chunks,
        //     &_chunk_size,
        // );


        // Set up our Vec of threads, so that we can await them
        let mut _threads:Vec<thread::JoinHandle<Vec<u64>>> = Vec::with_capacity(MAX_WORKERS);

        // A block to expire the cloned list of primes:
        {
            // While this is a clone of _list_of_primes, this should be a valid use because _list_of_primes needs to be mutable within this block.
            // We are also only doing this once every while segment, so not a lot.
            let _arc_of_primes:sync::Arc<Vec<u64>> = sync::Arc::new(_list_of_primes.clone());

            // DEBUG PRINT
            // Just to see how long it takes to clone this guy.
            // println!("Cloned {} items to an Arc.", _list_of_primes.len());

            for _chunk_id in 0.._no_of_chunks {
                let _list_of_primes_ref = sync::Arc::clone(&_arc_of_primes);

                let _chunk:Range<u64> = (_range_min + _chunk_id * _chunk_size)..cmp::min(_range_min + (_chunk_id+1) * _chunk_size, _range_max);

                // DEBUG PRINT
                // println!("Spawning thread for {:?}...", _chunk);

                // Creates a thread and push it to the Vec.
                _threads.push(
                    thread::spawn(move ||->Vec<u64> {
                        // println!("Chunk {:?} thread started.", &_chunk);
                        return list_primes_in_range_unthreaded(
                            _chunk,
                            &_list_of_primes_ref,
                        )
                    })
                );
            }

            // Now unwrap all the thread returns
            for _thread in _threads {
                let mut _thread_returned:Vec<u64> = _thread.join().unwrap();

                _list_of_primes.append(&mut _thread_returned);
            }
        }     

        // If you don't sort this you will soon discover 15 is a prime!
        _list_of_primes.sort();

        // DEBUG PRINT
        // println!("{} primes found up to {}.", _list_of_primes.len().to_formatted_string(&Locale::en), _range_max.to_formatted_string(&Locale::en))
    }
        
    
    return _list_of_primes;
}

/// Alias Function
pub fn list_primes(
    num:u64,
) -> Vec<u64> {
    return list_primes_threaded(num);
}