# -*- coding: utf-8 -*-
import pytest

import rust_primes


@pytest.mark.parametrize(
    ["num", "count"],
    [
        (0, 0),
        (2, 1),
        (3, 2),
        (4, 2),
        (5, 3),
        (10, 4),
        (10**2, 25),
        (10**3, 168),
        (10**4, 1_229),
        (10**5, 9_592),
        (10**6, 78_498),
        (10**7, 664_579),
        (10**8, 5_761_455),
        (10**9, 50_847_534),
    ],
)
@pytest.mark.parametrize(
    ["method"],
    ([rust_primes.SieveMethod.ATKIN], [rust_primes.SieveMethod.ERATOSTHENES]),
)
def test_count_primes(num, method, count):
    """
    Test if the prime counts are coorect.
    """
    assert rust_primes.count_primes(num, method=method) == count
