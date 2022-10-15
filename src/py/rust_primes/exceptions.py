# -*- coding: utf-8 -*-
"""
Exceptions for this package.
"""


class RustPrimesError(Exception):
    """
    The parent of all exceptions in this package.

    Evaluates to ``False``.
    """

    def __bool__(self) -> bool:
        return False

    __nonzero__ = __bool__


class BackendShellError(OSError, RustPrimesError):
    """
    A shell error occured during backend call.
    """


class BackendReturnError(ValueError, RustPrimesError):
    """
    The backend returned an unpicklable value.
    """
