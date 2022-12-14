pub mod base;
pub mod formulas;

pub use base::{
    is_prime,
    list_primes,
    count_primes,

    Sievable,
    SieveOfAtkin,
    SieveOfEratosthenes,
    SieveOfEratosthenesThreaded,
    // WheelFactorisedPrimeCheck,
};

pub use formulas::{
    upper_bound_of_nth_prime,
    list_n_primes,
    nth_prime,
};
