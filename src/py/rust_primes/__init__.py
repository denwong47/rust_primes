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

import functools

from . import lib_rust_primes as bin

SieveMethod = bin.SieveMethod
"""
Pseudo-Enum class to define method of prime sieving.

A pseudo-Enum class defined in Rust, this class is NOT an instance of the Python
:class:`enum.Enum` class, even if it behaves mostly in the same way.

There are currently two members available:

- :attr:`SieveMethod.ATKIN`: Modern method, but less well optimised by the compiler;
  not necessarily more performant.
- :attr:`SieveMethod.ERATOSTHENES`: The ancient method. Using the
  :meth:`ndarray.slice_mut().step` method, the compiler can optimise the inner loop
  to a close to ``O(n)`` operation. *This is the default.*
"""

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

list_primes = functools.lru_cache(bin.list_primes)
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

count_primes = functools.lru_cache(bin.count_primes)
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

upper_bound_of_nth_prime = functools.lru_cache(bin.upper_bound_of_nth_prime)
"""
Return the highest possible value of the nth prime.

The result is given as a :class:`int`.

Parameters
----------
n : int
    The ``n``-th prime to be estimated.

Returns
-------
int
    The upper bound of the ``n``-th prime.
"""

list_n_primes = functools.lru_cache(bin.list_n_primes)
"""
List the first ``n`` primes.

Parameters
----------
n : int
    The number of primes to return.

Returns
-------
List[int]
    A :class:`list` of the first ``n`` primes in :class:`int`.
"""

nth_prime = functools.lru_cache(bin.nth_prime)
"""
Find the ``n``-th prime.

Parameters
----------
n : int
    The ``n``-th prime to return.

Returns
-------
int
    The ``n``-th prime.

None
    If ``n`` is invalid (e.g. ``0``).
"""
