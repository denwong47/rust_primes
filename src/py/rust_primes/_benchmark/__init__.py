# -*- coding: utf-8 -*-
"""
Benchmarking utilities.

For speed testing only, no purposes in production.
"""
from .. import SieveMethod, count_primes


def run(
    max_power: int = 9,
    number: int = 6,
):
    """
    Run a primitive benchmark on the screen.

    Parameters
    ----------
    max_power : int
        The benchmarks will be run counting primes upto and including 10^`max_power`.

    count : int
        Number of times to repeat each test. The figure displayed will be the average of
        all tests.
    """
    for power in range(4, max_power + 1):
        print(f"Calculating all primes under 10^{power:d}:")

        for method in (
            SieveMethod.ATKIN,
            SieveMethod.ERATOSTHENES,
            SieveMethod.ERATOSTHENES_THREADED,
        ):
            print(
                f"- {str(method):40}: "
                f"{count_primes.timed(10**power, method=method, number=number).avg:.6f}"
                "s"
            )

        print()
