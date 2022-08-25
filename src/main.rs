#[macro_use]
extern crate timeit;

use num_format::{Locale, ToFormattedString};

// See math/mod.rs
mod math;


fn main() {
    const NUM_TO_TEST:u64 = 79;
    const MAX_RANGE:u64 = u64::pow(10,6);

    // Testing only; make sure math:is_prime is still working.
    // println!(
    //     "It is {} that {} is a prime number.",
    //         math::is_prime(NUM_TO_TEST),
    //         NUM_TO_TEST.to_formatted_string(&Locale::en)
    // );
    assert_eq!(
        math::is_prime(NUM_TO_TEST),
        true,
    );

    // This is where we will store our list
    let mut prime_list:Vec<u64> = Vec::new();

    // Let the race begin
    let sec = timeit_loops!(100, {
        prime_list = math::list_primes(MAX_RANGE);
    });

    println!(
        "Within the first {} numbers, there are {} prime numbers.",
            MAX_RANGE.to_formatted_string(&Locale::en),
            prime_list.len().to_formatted_string(&Locale::en)
    );
    
    println!(
        "Calculating all primes up to {} had taken {}s.",
            MAX_RANGE.to_formatted_string(&Locale::en),
            sec,
    );

    // Validating it using the "dumb", unthreaded version... which is somehow faster.
    let sec = timeit_loops!(1, {
        assert_eq!(
            prime_list,
            math::list_primes_unthreaded(MAX_RANGE)
        );
    });

    println!(
        "Validating all primes up to {} had taken {}s.",
            MAX_RANGE.to_formatted_string(&Locale::en),
            sec,
    );
}
