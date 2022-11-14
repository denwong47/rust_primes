use std::ops::BitAndAssign;
use std::{cmp, thread};
use std::sync::{Arc, Mutex};
use ndarray::{ArcArray1, Array, ArrayBase, Dim, OwnedRepr, s};
use rayon::prelude::*;

pub type Sieve = ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>>;

pub const BASE_WHEEL_SIZE:u64 = 64;
pub const MAX_WHEEL_SIZE:u64 = 10_u64.pow(7);
pub const DEFAULT_WORKERS_PROPORTION:f64 = 1.;
pub const DEFAULT_WORKERS:usize = 4;

#[allow(unused_imports)]
use timeit::{timeit_loops};
// Trait indicating a struct can perform a prime sieve.
pub trait Sievable {
    // This is NOT implemented below.
    // This is meant for the Enum, which will implement this trait,
    // from which the sieve function will call the Sieves below.
    fn sieve(&self, ubound:u64) -> Sieve;
}

/// Collect all the vectors from a Sieve.
/// Return as a Vec of all the prime integers.
pub fn collect(
    sieve: &Sieve,
    n_limit:Option<u64>,
) -> Vec<u64> {
    let result = sieve.iter()
                    .enumerate()
                    .filter(|&(_, &value)| value)
                    .map(|(index, _)| index as u64);

    return if let Some(n) = n_limit {
        result.take(n as usize).collect::<Vec<u64>>()
    } else {
        result.collect::<Vec<u64>>()
    }
}

/// Count the vectors in a Sieve.
/// Return as u64.
pub fn count(
    sieve: &Sieve,
) -> u64 {
    let result: u64 =  sieve.iter()
                            .filter(|&value| *value)
                            .count() as u64;

    return result;
}

/// Returns an `ndarray` of `bool` that indicates whether its index is a prime number.
/// Private function, used for `list_primes` and `count_primes`.
///
/// Using ndarray, the Sieve of Eratosthenes turns out to be the optimal method of calculating
/// primes. This method does not require threading; instead it relies on ndarray's efficiency when
/// using `slice_mut` over steps to achieve incredible speed.
///
/// This struct also implements wheel factorisation to optimise the numbers to check.
pub struct SieveOfEratosthenes;
impl SieveOfEratosthenes {
    pub fn sieve(
        ubound:u64,
    ) -> ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> {

        // Build inner wheel
        let sieve = Self::sieve_with_existing(
            ubound,
            &Array::from_vec(
                vec![false, false, true, true]
            ),
        );

        return sieve;
    }

    /// Perform the sieve by expanding an existing sieve
    pub fn sieve_with_existing(
        ubound:u64,
        sieve_input:&Sieve,
    ) -> Sieve {

        let mut sieve = Array::from_elem(((ubound+1) as usize, ), true);

        {
            let mut sieve_subset = sieve.slice_mut(s![..cmp::min(sieve_input.len(), sieve.len())]);
            sieve_subset.assign(&sieve_input.slice(s![..sieve_subset.len()]));
        }

        if sieve.len() > sieve_input.len() {
            for prime in 2..cmp::max(3, ((ubound+1) as f64).sqrt().ceil() as usize) {
                if sieve[prime] {
                    if prime as usize*2 >= sieve.len() { continue; }

                    let mut factors = sieve.slice_mut(s![prime*2..; prime]);

                    factors.fill(false);
                }
            }
        }

        return sieve;
    }

}

/// Returns an `ndarray` of `bool` that indicates whether its index is a prime number.
/// Private function, used for `list_primes` and `count_primes`.
///
/// Using ndarray, the Sieve of Atkin is the modern method of calculating
/// primes.
pub struct SieveOfAtkin;
impl SieveOfAtkin {
    pub fn sieve(
        ubound:u64,
    ) -> Sieve {

        let mut sieve = Array::from_elem(((ubound+1) as usize, ), false);

        // Set 2 and 3 to true
        if ubound >= 2 { sieve[2] = true }
        if ubound >= 3 { sieve[3] = true }

        let mut x:u64 = 1;

        while x.pow(2) <= ubound {
            let mut y:u64 = 1;

            while y.pow(2) <= ubound {
                let n:usize = (4*x.pow(2) + y.pow(2)) as usize;
                if  n <= ubound as usize
                    && (
                        n % 12 == 1
                        || n % 12 == 5
                    )
                {
                    sieve[n] = sieve[n] ^ true;
                }


                let n:usize = (3*x.pow(2) + y.pow(2)) as usize;
                if  n <= ubound as usize
                    && n % 12 == 7
                {
                    sieve[n] = sieve[n] ^ true;
                }


                let n:usize = (3*x.pow(2) - y.pow(2)) as usize;
                if  n <= ubound as usize
                    && x > y
                    && n % 12 == 11
                {
                    sieve[n] = sieve[n] ^ true;
                }

                y += 1;
            }

            x += 1;
        }

        let mut r:usize = 5;
        while r.pow(2) <= ubound as usize {
            if sieve[r] {
                let mut factors = sieve.slice_mut(
                    s![r.pow(2)..; r.pow(2)]
                );

                factors.fill(false);
            }

            r += 1;
        }

        return sieve;
    }
}


/// Experiment to make SieveOfEratosthenes threaded using rayon.
///
/// How it works:
///
/// For any given integer n, the smallest number which has a minimum factor
/// >n will be (n+1)*(n+1).
/// Therefore assuming we know all the primes up to value n: [2, 3, 5... k]
/// where k <= n, we can safely assert that all non-prime numbers up to (n+1)*(n+1)-1
/// will have at least one factor within our list.
///
/// Thus, for every n, we will take what we already know are primes: [2, 3, 5... k] and
/// check for factors in all values between n+1 to (n+1)*(n+1)-1. Now that we know all
/// the primes up to (n+1)*(n+1) (which we know for sure is NOT a prime), then we can
/// take (n+1)*(n+1) as the new n, then repeat.
///
/// i.e.
/// n = 2           We check all numbers between 3 to 3*3-1=8 with our prime
///                 list of [2],
/// yielding: [2, 3, 5, 7]
/// n = 9           We check all numbers between 10 to 10*10-1=99 with our prime
///                 list of [2, 3, 5, 7],
/// yielding: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
///            43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
/// n = 100         We check all numbers between 101 to 101*101-1=10202 with
///                 our prime list ...
///
/// We then stop when upper_limit is reached.
///
/// Typically this approach is very slow when n is low; set BASE_WHEEL_SIZE to higher
/// value to start with a larger initial list. The initial list will be generated by
/// the standard SieveOfEratosthenes.
pub struct SieveOfEratosthenesThreaded;
impl SieveOfEratosthenesThreaded {
    pub fn sieve(
        ubound:u64,
    ) -> ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> {

        let base_wheel_size = cmp::min(
            MAX_WHEEL_SIZE,
            cmp::max(
                BASE_WHEEL_SIZE,
                (ubound as f64).sqrt() as u64,
            )
        );

        // Build inner wheel
        let sieve = SieveOfEratosthenes::sieve(
            cmp::min(base_wheel_size, ubound),
        );

        if (ubound+1) as usize == sieve.len() && false {
            return sieve;
        } else {
            return Self::sieve_parallelised(ubound, &sieve);
        }

    }

    fn sieve_parallelised(
        ubound:u64,
        sieve_input:&Sieve,
    ) -> Sieve {
        if sieve_input.len() < ubound as usize {
            let mut sieve:Sieve = sieve_input.clone(); // Possibly cloning here.

            while sieve.len() < ubound as usize {
                // DEBUG PRINT
                // println!("Sieve loop operation: {}s", timeit_loops!(1, {

                let sieve_output = Self::sieve_chunk(
                    cmp::min(ubound, (sieve.len().pow(2)-1) as u64),
                    &sieve,
                );

                // Swap out the underlying value of sieve.
                *&mut sieve = sieve_output;

                // DEBUG PRINT
                // }));
            };

            return sieve;
        } else{
            // If the sieve is already less than
            return SieveOfEratosthenes::sieve_with_existing(
                ubound,
                sieve_input,
            );
        }

    }

    fn sieve_chunk(
        ubound:u64,
        sieve_input:&Sieve,
    ) -> Sieve {
        assert!(
            ubound < sieve_input.len().pow(2) as u64,
            "sieve_chunk can only be safely used with ubound < sieve_input.len().pow(2); found ubound={:?}, sieve_input.len()={:?}.",
            ubound,
            sieve_input.len(),
        );

        let mut sieve = Array::from_elem(((ubound+1) as usize, ), true);

        // Replace inner wheel with sieve_input
        {
            let mut sieve_subset = sieve.slice_mut(s![..cmp::min(sieve_input.len(), sieve.len())]);
            sieve_subset.assign(&sieve_input.slice(s![..sieve_subset.len()]));
        }

        if sieve.len() > sieve_input.len() {
            let sieve_mutex = Arc::new(Mutex::new(sieve));

            let workers = match thread::available_parallelism() {
                Ok(count) => (usize::from(count) as f64 * DEFAULT_WORKERS_PROPORTION).ceil() as usize,
                Err(_) => DEFAULT_WORKERS,
            };


            /*'_thread_block:*/ {
                let primes = ArcArray1::from_vec(collect(&sieve_input, None));

                let ubound_arc = Arc::from(ubound);

                // let chunk_size = (primes.len() as f64 / workers as f64).ceil() as usize;
                // let primes_chunk_iter = primes.axis_chunks_iter(Axis(0), chunk_size).into_par_iter();

                let _threads:Vec<()> = (0..workers)
                    .into_par_iter()
                    .map(
                        |worker_id| {
                            let sieve_mutex_ref = Arc::clone(&sieve_mutex);
                            let ubound_ptr = Arc::clone(&ubound_arc);
                            let prime_slice = primes.slice(s![worker_id..; workers]);

                            let mut sieve_thread = Array::from_elem(((*ubound_ptr+1) as usize,), true);

                            // DEBUG PRINT
                            // println!("Thread Operation on primes {}: {}s", &prime_slice, timeit_loops!(1, {
                            for prime in prime_slice {
                                if *prime as usize*2 < sieve_thread.len() {
                                    let lbound = *prime as usize * cmp::max(
                                        2,
                                        (sieve_input.len() as f64 / *prime as f64).ceil() as usize,
                                    );
                                    let mut sieve_slice = sieve_thread.slice_mut(s![lbound..; *prime as usize]);

                                    sieve_slice.fill(false);
                                }
                            };
                            // DEBUG PRINT
                            // }));

                            // DEBUG PRINT
                            // println!("Mutex block: {}s", timeit_loops!(1, {
                            /*'_mutex_block:*/ {
                                let mut sieve = sieve_mutex_ref.lock().unwrap();

                                sieve.bitand_assign(&sieve_thread);
                            }
                            // DEBUG PRINT
                            // }));
                        }
                    )
                    .collect();

                // DEBUG PRINT
                // println!("Sqashing all sieve results: {}s", timeit_loops!(1, {
                // for sieve_thread in &threads {
                //     sieve = sieve & sieve_thread;
                // };
                // DEBUG PRINT
                // }));
            }

            let sieve = Arc::try_unwrap(sieve_mutex)
                .unwrap()
                .into_inner()
                .unwrap();
            return sieve;
        } else {
            return sieve;
        }


    }
}
