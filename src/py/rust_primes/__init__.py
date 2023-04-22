# -*- coding: utf-8 -*-
"""
Utilities for prime calculations in Python using Rust backend.

This project includes a Rust binary backend:

- :mod:`lib_rust_primes` which can be loaded as
  ``from rust_primes import bin``.
"""

from . import decorators
from . import lib_rust_primes as bin  # pylint: disable=redefined-builtin # type: ignore

SieveMethod = bin.SieveMethod
"""
Pseudo-Enum class to define method of prime sieving.

A pseudo-Enum class defined in Rust, this class is NOT an instance of the Python
:class:`enum.Enum` class, even if it behaves mostly in the same way.

There are currently three members available:

- :attr:`SieveMethod.ATKIN`: Modern method, but less well optimised by the compiler;
  not necessarily more performant.
- :attr:`SieveMethod.ERATOSTHENES`: The ancient method. Using the
  :meth:`ndarray.slice_mut().step` method, the compiler can optimise the inner loop
  to a close to ``O(n)`` operation.
- :attr:`SieveMethod.ERATOSTHENES_ATOMIC`: Same method as :attr:`ERATOSTHENES`, but
  conducted on an array of Atomic booleans instead. Allows threading to run without
  much overhead. *This is the default.*
- :attr:`SieveMethod.ERATOSTHENES_THREADED`: *Experimental*. An attempt to introduce
  threading into :attr:``ERATOSTHENES``. It works by using :attr:`ERATOSTHENES` to
  create a base array of primes, upto :func:`Math.sqrt` of the upper bound, then
  split the list of primes into equal size for each worker, then each worker sieve the
  rest of the (larger) numbers.
"""

is_prime = decorators.TimedFunction(bin.is_prime)
"""
Check if the given number is prime.

This function checks if a given number is a prime number, and returns a :class:`bool`
indicating the result.

.. seealso::
    While this function behaves like a function, it is implemented through
    :class:`~rust_primes.decorators.timed.TimedFunction` and thus contains additional
    functionalities with caching and
    :meth:`~rust_primes.decorators.timed.TimedFunction.timed` executions.

    See :class:`rust_primes.decorators.timed.TimedFunction` for more details.

Parameters
----------
num : int
    The number to be checked.

method : SieveMethod
    The method of sieving to be used for finding the primes.

Returns
-------
bool
    ``True`` if prime, ``False`` otherwise.
"""

list_primes = decorators.TimedFunction(bin.list_primes)
"""
List all primes numbers less than or equal to ``num``.

The result is given in a ``List[int]``. Due to the FFI exchange of data between
Rust and Python, this function will be somewhat slower than the
:func:`count_primes` implementation. If you intend to ``len(list_primes(n))``,
use :func:`count_primes` instead.

.. seealso::
    While this function behaves like a function, it is implemented through
    :class:`~rust_primes.decorators.timed.TimedFunction` and thus contains additional
    functionalities with caching and
    :meth:`~rust_primes.decorators.timed.TimedFunction.timed` executions.

    See :class:`rust_primes.decorators.timed.TimedFunction` for more details.

Parameters
----------
num : int
    The upper bound to be checked.

method : SieveMethod
    The method of sieving to be used for finding the primes.

Returns
-------
List[int]
    List of all primes, starting from 2, up to and including ``num``.
"""

count_primes = decorators.TimedFunction(bin.count_primes)
"""
Count the number of primes numbers less than or equal to ``num``.

The result is given as a :class:`int`.

.. seealso::
    While this function behaves like a function, it is implemented through
    :class:`~rust_primes.decorators.timed.TimedFunction` and thus contains additional
    functionalities with caching and
    :meth:`~rust_primes.decorators.timed.TimedFunction.timed` executions.

    See :class:`rust_primes.decorators.timed.TimedFunction` for more details.

Parameters
----------
num : int
    The upper bound to be checked.

method : SieveMethod
    The method of sieving to be used for finding the primes.

Returns
-------
int
    Number of prime numbers up to and including ``num``.
"""

upper_bound_of_nth_prime = decorators.TimedFunction(bin.upper_bound_of_nth_prime)
"""
Return the highest possible value of the nth prime.

The result is given as a :class:`int`.

.. note::
    This function does NOT have a :attr:`method` parameter.

.. seealso::
    While this function behaves like a function, it is implemented through
    :class:`~rust_primes.decorators.timed.TimedFunction` and thus contains additional
    functionalities with caching and
    :meth:`~rust_primes.decorators.timed.TimedFunction.timed` executions.

    See :class:`rust_primes.decorators.timed.TimedFunction` for more details.

Parameters
----------
n : int
    The ``n``-th prime to be estimated.

Returns
-------
int
    The upper bound of the ``n``-th prime.
"""

list_n_primes = decorators.TimedFunction(bin.list_n_primes)
"""
List the first ``n`` primes.

.. seealso::
    While this function behaves like a function, it is implemented through
    :class:`~rust_primes.decorators.timed.TimedFunction` and thus contains additional
    functionalities with caching and
    :meth:`~rust_primes.decorators.timed.TimedFunction.timed` executions.

    See :class:`rust_primes.decorators.timed.TimedFunction` for more details.

Parameters
----------
n : int
    The number of primes to return.

method : SieveMethod
    The method of sieving to be used for finding the primes.

Returns
-------
List[int]
    A :class:`list` of the first ``n`` primes in :class:`int`.
"""

nth_prime = decorators.TimedFunction(bin.nth_prime)
"""
Find the ``n``-th prime.

.. seealso::
    While this function behaves like a function, it is implemented through
    :class:`~rust_primes.decorators.timed.TimedFunction` and thus contains additional
    functionalities with caching and
    :meth:`~rust_primes.decorators.timed.TimedFunction.timed` executions.

    See :class:`rust_primes.decorators.timed.TimedFunction` for more details.

Parameters
----------
n : int
    The ``n``-th prime to return.

method : SieveMethod
    The method of sieving to be used for finding the primes.

Returns
-------
int
    The ``n``-th prime.

None
    If ``n`` is invalid (e.g. ``0``).
"""
