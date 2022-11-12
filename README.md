# rust_primes

Utilities for prime calculations in Python using Rust backend.

This project includes a Rust binary backend:

* `lib_rust_primes` which can be loaded as `rust_primes.bin`.

### Subpackages

* `rust_primes.config` package

  * `rust_primes.config.env` module

    * Environment Definitions


## Functions

### Function `rust_primes.is_prime()`

Check if the given number is prime.

This function checks if a given number is a prime number, and
returns a "bool" indicating the result.

- Parameters:
    - `*num*` (`int`) – The number to be checked.

- Returns:
    - "True" if prime, "False" otherwise.

- Return type:
    - bool

### Function `rust_primes.list_primes()`

   - List all primes numbers less than or equal to "num".

   - The result is given in a "List[int]".

   - Parameters:
      - `*num*` (`int`) – The upper bound to be checked.

   - Returns:
      - List of all primes, starting from 2, up to and including "num".

   - Return type:
      - List[int]

### Function `rust_primes.count_primes()`

   - Count the number of primes numbers less than or equal to "num".

   - The result is given as a "int".

   - Parameters:
      - `*num*` (`int`) – The upper bound to be checked.

   - Returns:
      - Number of prime numbers up to and including "num".

   - Return type:
      - int


## `rust_primes.bin` module

* Alias for `lib_rust_primes` module *

The backend functions, implemented in Rust.

### Function `rust_primes.lib_rust_primes.is_prime()`

   Rust library function.

   Determines if a number is a prime number. This is a wrapper around
   `is_prime_with_known_primes`, which actually does the work. The
   difference is that `is_prime_with_known_primes` requires a list of
   all known primes to be passed to it, up to and including the `sqrt`
   of the number itself. `is_prime` will enquire that list using
   `list_primes` first, before calling `is_prime_with_known_primes`.

### Function `rust_primes.lib_rust_primes.list_primes()`

   Rust library function.

   List all the primes within `ubound`. Calls `prime_mask`, and apply
   the mask on `enumerate`.

### Function `rust_primes.lib_rust_primes.count_primes()`

   Rust library function.

   Return the number of primes within `ubound`. Calls `prime_mask`,
   and filter it by `true` before getting its `len`.
