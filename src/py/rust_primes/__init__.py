# -*- coding: utf-8 -*-
"""
================================
 Python Prime Numbers Utilities
================================

This utilities uses a Rust backend, in the ``src/rust/`` directory, to do prime number calculations.

This allows true threading without Python GIL getting in the way.

The primary public functions are:

- :func:`~rust_primes.func.is_prime` - Check if a number is prime;
- :func:`~rust_primes.func.list_primes` - Return a list of primes within a provided value;
- :func:`~rust_primes.func.count_primes` - Count the number of primes within a provided value.
"""
from . import exceptions
from .backend import BACKEND_PATH
from .func import count_primes, is_prime, list_primes
