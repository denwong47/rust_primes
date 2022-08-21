use num_format::{Locale, ToFormattedString};

mod math;


fn main() {
    const NUM_TO_TEST:u64 = 79;
    const MAX_RANGE:u64 = 1000000;

    println!(
        "It is {} that {} is a prime number.",
            math::is_prime(NUM_TO_TEST),
            NUM_TO_TEST.to_formatted_string(&Locale::en)
    );
    println!(
        "Within the first {} numbers, there are {} prime numbers.",
            MAX_RANGE.to_formatted_string(&Locale::en),
            math::list_primes(MAX_RANGE).len().to_formatted_string(&Locale::en)
    );
}
