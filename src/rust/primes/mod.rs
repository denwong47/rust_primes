pub mod base;
pub mod formulas;

pub use base::{
    count_primes,

    is_prime,
    list_primes,
    Sievable,
    SieveOfAtkin,
    SieveOfEratosthenes,
    SieveOfEratosthenesThreaded,
    // WheelFactorisedPrimeCheck,
};

pub use formulas::{list_n_primes, nth_prime, upper_bound_of_nth_prime};
