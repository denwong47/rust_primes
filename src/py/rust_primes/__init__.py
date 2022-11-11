# -*- coding: utf-8 -*-
"""
================================
 rust_primes
================================

Utilities for prime calculations in Python using Rust backend.

This project includes a Rust binary backend:

- :mod:`lib_rust_primes` which can be loaded as
  :attr:`~rust_primes.bin`.
"""

from . import lib_rust_primes as bin

is_prime = bin.is_prime
"""
Check if the given number is prime.

This function checks if a given number is a prime number, and returns a :class:`bool`
indicating the result.

Parameters
----------
num : int
    The number to be checked.

Returns
-------
bool
    ``True`` if prime, ``False`` otherwise.
"""

list_primes = bin.list_primes
"""
List all primes numbers less than or equal to ``num``.

The result is given in a ``List[int]``.

Parameters
----------
num : int
    The upper bound to be checked.

Returns
-------
List[int]
    List of all primes, starting from 2, up to and including ``num``.
"""

count_primes = bin.count_primes
"""
Count the number of primes numbers less than or equal to ``num``.

The result is given as a :class:`int`.

Parameters
----------
num : int
    The upper bound to be checked.

Returns
-------
int
    Number of prime numbers up to and including ``num``.
"""

upper_bound_of_nth_prime = bin.upper_bound_of_nth_prime
"""
Return the highest possible value of the nth prime.

The result is given as a :class:`int`.

Parameters
----------
n : int
    The ``n``th prime to be estimated.

Returns
-------
int
    The upper bound of the ``n``th prime.
"""
