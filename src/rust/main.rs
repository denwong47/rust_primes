#[macro_use]
extern crate timeit;

use std::env;
use num_format::{Locale, ToFormattedString};

// See math/mod.rs
mod math;
mod options;

use options::output::Output;
use options::args::CommandLineArgs;

fn main() {

    const DEFAULT_MAX_RANGE:u64 = u64::pow(10,6);

    let mut print_time:bool = false;

    fn execute(print_time:&mut bool) {
        let args: Vec<String> = env::args().collect();

        match CommandLineArgs::from_args(
            &args,
            CommandLineArgs::Count(DEFAULT_MAX_RANGE, Output::Readable),
        ) {
            CommandLineArgs::Check(value, output) => {
                let result = math::is_prime(value);
                match output {
                    Output::Readable => {
                        *print_time = true;
                        println!(
                            "{} is {}a prime number.",
                            value,
                            match result {
                                true => "",
                                false => "NOT ",
                            }
                        );
                    },
                    _ => output.display(&result),
                }
            },

            CommandLineArgs::Count(value, output) => {
                let prime_list = math::list_primes(value);
                match output {
                    Output::Readable => {
                        *print_time = true;
                        println!(
                            "Within the first {} numbers, there are {} prime numbers.",
                            value.to_formatted_string(&Locale::en),
                            prime_list.len().to_formatted_string(&Locale::en)
                        )
                    },
                    _ => output.display(&prime_list),
                }
            },

            CommandLineArgs::List(value, output) => {
                let prime_list = math::list_primes(value);
                match output {
                    Output::Readable => {
                        *print_time = true;
                        println!(
                            "Prime numbers within {}:\n{:?}",
                            value.to_formatted_string(&Locale::en),
                            prime_list
                        )
                    },
                    _ => output.display(&prime_list),
                }
            },

            CommandLineArgs::Invalid => {
                println!(
                    "Invalid command input."
                )
            }
        }
    }

    let sec = timeit_loops!(1, {execute(&mut print_time)});

    if print_time {
        println!(
            "Execution had taken {:.6}s.",
            sec,
        );
    }

    // // Validating it using the "dumb", unthreaded version... which is somehow faster.
    // let sec = timeit_loops!(1, {
    //     assert_eq!(
    //         prime_list,
    //         math::list_primes_unthreaded(DEFAULT_MAX_RANGE)
    //     );
    // });

    // println!(
    //     "Validating all primes up to {} had taken {}s.",
    //         DEFAULT_MAX_RANGE.to_formatted_string(&Locale::en),
    //         sec,
    // );
}
