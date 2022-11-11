# -*- coding: utf-8 -*-
from typing import List

import pytest

import rust_primes

MAX_POWER: int = 9
MAX_LOG10_N: int = 7


@pytest.fixture
def all_primes() -> List[int]:
    # Don't worry, this is lru_cached
    return rust_primes.list_primes(10**MAX_POWER)


@pytest.mark.parametrize(["n"], ([10**power] for power in range(MAX_LOG10_N + 1)))
def test_upper_bound(n, all_primes):
    """
    Test if the upper bounds is correct.

    10**8 is chosen because it goes through all 4 modes of the calculation:
    - n < 6:                    Literal
    - 6 <= n < 688383:          Rosser-Schoenfeld
    - 688383 <= n < 8009824:    Dusart, 2010
    - n >= 8009824:             Korollar G.
    """
    assert rust_primes.upper_bound_of_nth_prime(n) >= all_primes[n - 1]


@pytest.mark.parametrize(["n"], ([10**power] for power in range(MAX_LOG10_N + 1)))
@pytest.mark.parametrize(
    ["method"],
    ([rust_primes.SieveMethod.ATKIN], [rust_primes.SieveMethod.ERATOSTHENES]),
)
def test_list_n_primes(n, method, all_primes):
    """
    Test if the list of first ``n`` primes are matching the overall list.
    """
    _n_primes = rust_primes.list_n_primes(n, method=method)
    assert len(_n_primes) == n
    assert _n_primes == all_primes[:n]
