pub mod base;
pub mod formulas;

pub use base::{
    is_prime,
    list_primes,
    count_primes,
};

pub use formulas::{
    upper_bound_of_nth_prime,
};
