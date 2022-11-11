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

print ("### Init had run ###")