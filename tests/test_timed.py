# -*- coding: utf-8 -*-
from timeit import timeit

import pytest

import rust_primes

REPEAT_EXECUTIONS = 12


@pytest.mark.parametrize(
    ["num", "count"],
    [
        (10**7, 664_579),
    ],
)
@pytest.mark.parametrize(
    ["method"],
    ([rust_primes.SieveMethod.ATKIN], [rust_primes.SieveMethod.ERATOSTHENES]),
)
def test_timed_execution(num, count, method):
    """
    Testing timed execution being uncached and is returning the correct results.

    If the subsequent executions are cached, it will be ~0s compared to the initial
    calculation. Since we are repeating the execution 12 times, the average exeuction
    time when cached is going to be disproportionately lower than normal, way more
    than the 40% allowed here.
    """
    # Single execution without using any of :class:`TimedFunction` utilities
    _single_execution = timeit(
        lambda: rust_primes.count_primes._func(num=num, method=method), number=1
    )
    _multiple_execution = rust_primes.count_primes.timed(
        num=num, number=REPEAT_EXECUTIONS, method=method
    )

    # Ensure results are correct
    assert count == _multiple_execution.result

    # Ensure that :attr:`avg` is working
    assert _multiple_execution.avg == _multiple_execution.duration / REPEAT_EXECUTIONS

    # Ensure that the execution is uncached.
    assert _multiple_execution.avg == pytest.approx(
        _single_execution,
        rel=0.4,
    )
