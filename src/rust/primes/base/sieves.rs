use std::{cmp};
use ndarray::{Array, ArrayBase, Dim, OwnedRepr, s};

// Trait indicating a struct can perform a prime sieve.
pub trait Sievable {
    // This is NOT implemented below.
    // This is meant for the Enum, which will implement this trait,
    // from which the sieve function will call the Sieves below.
    fn sieve(&self, ubound:u64) -> ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>>;
}

/// Returns an `ndarray` of `bool` that indicates whether its index is a prime number.
/// Private function, used for `list_primes` and `count_primes`.
///
/// Using ndarray, the Sieve of Eratosthenes turns out to be the optimal method of calculating
/// primes. This method does not require threading; instead it relies on ndarray's efficiency when
/// using `slice_mut` over steps to achieve incredible speed.
pub struct SieveOfEratosthenes;
impl SieveOfEratosthenes {
    pub fn sieve(
        ubound:u64,
    ) -> ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> {

        let mut sieve = Array::from_elem(((ubound+1) as usize, ), true);

        // Set 0 and 1 to false
        sieve.slice_mut(s![0..cmp::min(2, sieve.len())]).fill(false);

        // Starting from the 2, we sieve through all the prime numbers and mark them as non-primes.
        // Note that we have to make sure that 2 will be checked at the very least, otherwise no primes
        // will be calculated up to prime_mask(7)!
        if ubound > 2 {
            for prime in 2..cmp::max(3, ((ubound+1) as f64).sqrt().ceil() as usize) {
                if sieve[prime] {

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
    ) -> ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> {

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
