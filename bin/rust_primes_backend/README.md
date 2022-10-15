# Rust Backend
This folder contains the compiled binaries from Rust.

Python will always use the `release` variant of `rust_primes`.

To rebuild, run in the root directory:

```sh
cargo build --release
```

## Example use for `rust_primes` binary:

```sh
rust_primes check 79        # Check if a number is prime
rust_primes count 1000      # Count the number of primes <= 1000
rust_primes list 1000       # List all the primes <=1000 in [2, 3, 5... ] format

# For use as JSON
rust_primes list 1000 raw    # Remove all readable strings and print result only.

# For use with Python
rust_primes list 1000 pickle # Remove all readable strings and print result as Python compatible pickled data.
```
