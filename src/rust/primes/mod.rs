pub mod constructs;
pub mod formulas;
pub(crate) mod methods;

pub use methods::{
    SieveOfAtkin,
    SieveOfEratosthenes,
    SieveOfEratosthenesAtomic,
    SieveOfEratosthenesThreaded,
    // WheelFactorisedPrimeCheck,
};

pub use formulas::{list_n_primes, nth_prime, upper_bound_of_nth_prime};
