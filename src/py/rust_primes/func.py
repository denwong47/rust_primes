# -*- coding: utf-8 -*-
"""
Defining the actual high level functions.
"""
import functools

from . import backend
from .backend import CommandType

count_primes = functools.partial(backend.call, command=CommandType.COUNT)
"""
Count the number of prime numbers less than or equal to ``value``.

Parameters
----------
value : int
    The :class:`int` value to be used with the command.

Returns
-------
int
    The number of prime numbers less than or equal to ``value``.
"""

list_primes = functools.partial(backend.call, command=CommandType.LIST)
"""
Return a list of all prime numbers less than or equal to ``value``.

Parameters
----------
value : int
    The :class:`int` value to be used with the command.

Returns
-------
List[int]
    A :class:`list` of all prime numbers less than or equal to ``value``.
"""

is_prime = functools.partial(backend.call, command=CommandType.CHECK)
"""
Check if the provided :class:`int` is a prime number.

Parameters
----------
value : int
    The :class:`int` value to be used with the command.

Returns
-------
bool
    ``True`` if the number is a prime number; otherwise ``False``.
"""
