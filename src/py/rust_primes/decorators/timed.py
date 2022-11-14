# -*- coding: utf-8 -*-
"""
Timed decorators for functions.

Works with any Python callables, but this is primarily built for Rust imported
functions.
"""
import functools
from timeit import timeit
from typing import Any, Callable, ClassVar


class TimedResult(float):
    """
    An extended :class:`float` class that can also store the result of execution.

    This is for the purpose of :meth:`TimedFunction.timed` so that it can return both
    the duration of execution as well as its results.

    Parameters
    ----------
    value : Any
        The value to be converted to `float`.

    number : int
        Number of times that execution has been run.

    result : Any
        Optional. Result from the last execution of :class:`TimedFunction`.
    """

    number: int
    """
    The number of executions run.
    """

    result: Any
    """
    The return value of the *last* execution.
    """

    def __new__(
        # pylint: disable-next=unused-argument
        cls,
        value: Any,
        *,
        number: int = 1,
        result: Any = None
    ) -> None:
        return super().__new__(cls, value)

    def __init__(
        # pylint: disable-next=unused-argument
        self,
        value: Any,
        *,
        number: int = 1,
        result: Any = None
    ) -> None:
        super().__init__()
        self.number = number
        self.result = result

    @property
    def duration(self) -> float:
        """
        Alias for for ``float(self)``.

        Returns the duration of execution recorded when this instance initiated.

        .. note::
            This returns a standard :class:`float` instance.
        """
        return float(self)

    @property
    def avg(self) -> "TimedResult":
        """
        Returns the average duration of execution for :class:`TimedFunction`.
        """
        return type(self)(self.duration / self.number, number=1, result=self.result)


class TimedFunction:
    """
    Decorator to add a `timed` method to the function.

    When decorated, functions have additional method calls that allows different
    behaviours::

        >>> import rust_primes

        >>> # Normal function call
        >>> rust_primes.count_primes(1000)
        ... 168

        >>> # If we make the same call again, the answer is cached this time
        >>> rust_primes.count_primes(1000)
        ... 168

        >>> # Timed function call
        >>> # While it does not look like it, the actual returned value is a TimedResult
        ...   object. It is a subclass of float which is what is displayed here,
        ...   but it also contains the return value etc.
        >>> # All parameters will be passed through to the underlying function.
        >>> _result = rust_primes.count_primes.timed(1000)
        >>> _result
        ... 1.360991392284632e-06

        >>> _result.result
        >>> 168

        >>> # Additional parameters
        >>> # `number` can be specified to state how many times the exeuction should be
        ...   run and timed.
        >>> _result = rust_primes.count_primes.timed(1000, number=10)
        ... 0.0016299039998557419

        >>> # Average run time
        >>> _result.avg
        ... 1.461696985643357e-06


    """

    _func: ClassVar[Callable]
    _last_returned: Any = None

    def __init__(self, func: Callable) -> None:
        self._func = func

    def uncached_call(self, *args: Any, **kwargs: Any) -> Any:
        """
        Call the underlying function without :func:`functools.lru_cache`.

        Parameters
        ----------
        args
            Parameters to be passed to the underlying function.

        kwargs
            Keyworded Parameters to be passed to the underlying function.

        Returns
        -------
        Any
            The return from the underlying function.
        """
        # Cache the last execution result
        self._last_returned = self._func(*args, **kwargs)
        return self._last_returned

    cached_call = functools.lru_cache(uncached_call)
    """
     Call the underlying function with ``lru_cache``.

        Parameters
        ----------
        *args, **kwargs
            Parameters to be passed to the underlying function.

        Returns
        -------
        Any
            The return from the underlying function.
    """

    # Default to cached call.
    __call__ = cached_call

    def timed(self, *args: Any, number: int = 1, **kwargs) -> TimedResult:
        """
        Run the underlying function, timed.

        When running :meth:`timed`, the function is always uncached. It will repeat
        the function call ``number`` of times, and the returned :class:`TimedResult` -
        which is a subclass of :class:`float`, will be the *TOTAL* time for all the
        executions.

        Parameters
        ----------
        *args, **kwargs
            Parameters to be passed to the underlying function.

        number : int
            Number of executions to be run.

        Returns
        ------
        TimedResult


        .. seealso::
            :class:`TimedResult` on how to retrieve the result and average run time.
        """
        _duration = timeit(
            # When timing something, do not cache anything.
            lambda: self.uncached_call(*args, **kwargs),
            number=number,
        )

        return TimedResult(
            _duration,
            number=number,
            result=self._last_returned,
        )
