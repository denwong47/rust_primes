# -*- coding: utf-8 -*-
"""
Subprocess controller.
"""

import enum
import functools
import pickle
import subprocess
from importlib import resources
from typing import List, Literal, Tuple, Union

from . import config, exceptions

BACKEND_PATH = resources.files("rust_primes_backend.release").joinpath("rust_primes")


class CommandType(enum.Enum):
    """
    Type of command to send to the backend.
    """

    COUNT = "count"
    LIST = "list"
    CHECK = "check"

    def __str__(self) -> str:
        return self.value

    @classmethod
    @functools.lru_cache()
    def values(cls) -> Tuple[str]:
        """
        Return a tuple containg all the :class:`str` values of this :class:`Enum`.
        """
        return tuple(map(str, cls._value2member_map_))


def call(
    value: int,
    *,
    command: Union[CommandType, Literal.__getitem__(CommandType.values())],
) -> Union[List[int], int]:
    """
    Call the underlying backend.

    Parameters
    ----------
    value : int
        The :class:`int` value to be used with the command.

    command : Union[CommandType, Literal["count", "list", "check"] ]
        The type of command to be executed.

    Returns
    -------
    int
        If command is ``count``: the number of prime numbers less than or equal to
        ``value``.

    bool
        If command is ``check``: whether ``value`` is a prime number.

    List[int]
        If command is ``list``: list of prime numbers less than or equal to ``value``.
    """
    assert str(command).lower() in CommandType.values(), (
        f"{repr(command)} is not an acceptable command for the backend. Expected one of "
        + ", ".join(map(repr, CommandType.values()))
    )

    assert isinstance(value, int) and value > 0, (
        "Value must be an positive `int` within the bounds of `u64`. "
        f"{repr(value)} found."
    )

    try:
        _output = subprocess.check_output(
            [BACKEND_PATH, str(command), str(value), "pickle"]
        )
        return pickle.loads(_output)

    except subprocess.SubprocessError as exception:
        raise exceptions.BackendShellError(
            f"Backend had an unhandled {type(exception).__name__}: {exception}"
        )

    except pickle.UnpicklingError as exception:
        raise exceptions.BackendReturnError(
            f"Backend return value is corrupted: {repr(_output[:1024])}..."
        )


if not config.DISABLE_CACHE:
    call = functools.lru_cache()(call)
