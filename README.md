# rust_primes

![CI Checks](https://github.com/denwong47/rust_primes/actions/workflows/CI.yml/badge.svg?branch=main)

Utilities for prime calculations in Python using Rust backend.

This project includes a Rust binary backend:

* `lib_rust_primes` which can be loaded as `rust_primes.bin`.

### Subpackages

* `rust_primes.config` package

  * `rust_primes.config.env` module

    * Environment Definitions


## Classes

### Enum `rust_primes.SieveMethod`

Pseudo-Enum class to define method of prime sieving.

A pseudo-Enum class defined in Rust, this class is NOT an instance of the Python
`enum.Enum` class, even if it behaves mostly in the same way.

This method is `hash`able. `__hash__` is implemented in Rust.

There are currently two members available:

- `SieveMethod.ATKIN`: Modern method, but less well optimised by the compiler;
  not necessarily more performant.
- `SieveMethod.ERATOSTHENES`: The ancient method. Using the
  `ndarray.slice_mut().step` method, the compiler can optimise the inner loop
  to a close to ``O(n)`` operation. *This is the default.*


## Functions

### Function `rust_primes.is_prime()`

Check if the given number is prime.

This function checks if a given number is a prime number, and
returns a "bool" indicating the result.

- Parameters:
    - `num` (`int`) – The number to be checked.
    - `method` (`SieveMehthod`) - The method of sieving to be used for finding the
      primes.

- Returns:
    - `True` if prime, `False` otherwise.

- Return type:
    - `bool`

### Function `rust_primes.list_primes()`

List all primes numbers less than or equal to "num".

The result is given in a `List[int]`.

   - Parameters:
      - `num` (`int`) – The upper bound to be checked.

   - Returns:
      - List of all primes, starting from 2, up to and including `num`.

   - Return type:
      - `List[int]`

### Function `rust_primes.count_primes()`

Count the number of primes numbers less than or equal to "num".

The result is given as a `int`.

   - Parameters:
      - `num` (`int`) – The upper bound to be checked.

   - Returns:
      - Number of prime numbers up to and including `num`.

   - Return type:
      - `int`


# `rust_primes.bin` module

*Alias for `lib_rust_primes` module*

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
