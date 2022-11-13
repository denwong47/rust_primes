use std::{cmp};
use ndarray::{Array, ArrayBase, Dim, OwnedRepr, s};

pub type Sieve = ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>>;

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
            cmp::min(1024, ubound),
            &Array::from_vec(
                vec![false, false, true, true, false]
            ),
        );

        return Self::sieve_with_existing_iterative(ubound, &sieve);
    }

    /// Perform the sieve by expanding an existing sieve
    pub fn sieve_with_existing(
        ubound:u64,
        sieve_input:&Sieve,
    ) -> Sieve {
        let mut sieve = Self::wheel_from_sieve(ubound, &sieve_input);

        if sieve.len() > sieve_input.len() {
            for prime in 2..cmp::max(3, ((ubound+1) as f64).sqrt().ceil() as usize) {
                if sieve[prime] {
                    if prime as usize*2 >= sieve.len() { break; }

                    let mut factors = sieve.slice_mut(s![prime*2..; prime]);

                    factors.fill(false);
                }
            }
        }

        return sieve;
    }

    /// Generate a wheel factorised sieve.
    /// The remaining numbers in the wheel are mostly prime numbers (they are collectively called "relatively" prime).
    /// Use sieve_with_existing or further wheel_from_sieve to remove the remaining non-primes.
    fn wheel_from_sieve(
        ubound:u64,
        sieve_input:&Sieve,
    ) -> Sieve {
        let mut sieve = Array::from_elem(((ubound+1) as usize, ), true);

        // Replace inner wheel with sieve_input
        {
            let mut sieve_subset = sieve.slice_mut(s![..cmp::min(sieve_input.len(), sieve.len())]);
            sieve_subset.assign(&sieve_input.slice(s![..sieve_subset.len()]));
        }

        // Work through each slice of the wheel, radiating from the inner wheel.
        if sieve.len() > sieve_input.len() {
            // Possible to insert an unsafe block here to thread this - because the slices don't overlap.
            let primes = collect(&sieve_input, None);
            let wheel_size = primes.iter().fold(1, |x, y| { x*y }) as usize;

            for prime in primes {
                if prime as usize + wheel_size >= sieve.len() { break; }

                let mut sieve_slice = sieve.slice_mut(s![prime as usize + wheel_size..; wheel_size]);

                sieve_slice.fill(false);
            }
        }

        return sieve;
    }

    /// Perform the sieve iteratively, maintaining <`max_layers` layers each time.
    /// This narrows the gaps when the layers are too thick.
    pub fn sieve_with_existing_iterative(
        ubound:u64,
        sieve_input:&Sieve,
    ) -> Sieve {

        // let n_layers = max_layers.or(Some(DEFAULT_MAX_LAYERS)).unwrap();

        let mut sieve:Sieve = sieve_input.clone(); // Possibly cloning here.

        if sieve.len() < ubound as usize {
            while sieve.len() < ubound as usize {
                let sieve_output = Self::sieve_with_existing(
                    cmp::min(ubound, (sieve.len().pow(2)) as u64),
                    &sieve,
                );

                // Swap out the underlying value of sieve.
                *&mut sieve = sieve_output;
            };
        } else{
            // If the sieve is already less than
            return Self::sieve_with_existing(
                ubound,
                &sieve,
            );
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
