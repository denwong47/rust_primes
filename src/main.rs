#[macro_use]
extern crate timeit;

use num_format::{Locale, ToFormattedString};

mod math;


fn main() {
    const NUM_TO_TEST:u64 = 79;
    const MAX_RANGE:u64 = 100000;

    println!(
        "It is {} that {} is a prime number.",
            math::is_prime(NUM_TO_TEST),
            NUM_TO_TEST.to_formatted_string(&Locale::en)
    );

    let mut prime_list:Vec<u64> = Vec::new();

    let sec = timeit_loops!(1, {
        prime_list = math::list_primes(MAX_RANGE);
    });

    println!(
        "Within the first {} numbers, there are {} prime numbers.",
            MAX_RANGE.to_formatted_string(&Locale::en),
            prime_list.len().to_formatted_string(&Locale::en)
    );
    // println!(
    //     "{:?}",
    //     prime_list
    // );
    println!(
        "Calculating all primes up to {} had taken {}s.",
            MAX_RANGE.to_formatted_string(&Locale::en),
            sec,
    );

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
