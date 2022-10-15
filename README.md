Python Prime Numbers Utilities
==============================

This utilities uses a Rust backend, in the "src/rust/" directory, to
do prime number calculations.

This allows true threading without Python GIL getting in the way.

The primary public functions are:

* "is_prime()" - Check if a number is prime;

* "list_prime()" - Return a list of primes within a provided value;

* "count_prime()" - Count the number of primes within a provided
  value.


rust_primes.backend module
==========================

Subprocess controller.

## class rust_primes.backend.CommandType(value)

   Bases: "Enum"

   Type of command to send to the backend.

   COUNT = 'count'

   LIST = 'list'

   CHECK = 'check'

   classmethod values() -> Tuple[str]

      Return a tuple containg all the "str" values of this "Enum".

## rust_primes.backend.call(value: int, *, command: Union[CommandType, Literal['count', 'list', 'check']]) -> Union[List[int], int]

   Call the underlying backend.

   Parameters:
      * **value** (*int*) -- The "int" value to be used with the
        command.

      * **command** (*Union**[**CommandType**,
        **Literal**[**"count"**, **"list"**, **"check"**] **]*) -- The
        type of command to be executed.

   Returns:
      * *int* -- If command is "count": the number of prime numbers
        less than or equal to "value".

      * *bool* -- If command is "check": whether "value" is a prime
        number.

      * *List[int]* -- If command is "list": list of prime numbers
        less than or equal to "value".


rust_primes.exceptions module
=============================

Exceptions for this package.

## exception rust_primes.exceptions.RustPrimesError

   Bases: "Exception"

   The parent of all exceptions in this package.

   Evaluates to "False".

## exception rust_primes.exceptions.BackendShellError

   Bases: "OSError", "RustPrimesError"

   A shell error occured during backend call.

## exception rust_primes.exceptions.BackendReturnError

   Bases: "ValueError", "RustPrimesError"

   The backend returned an unpicklable value.


rust_primes.func module
=======================

Defining the actual high level functions.

## rust_primes.func.count_primes(value: int, *, command: Union[CommandType, Literal['count', 'list', 'check']] = CommandType.COUNT) -> Union[List[int], int]

    Count the number of prime numbers less than or equal to "value".

    Parameters:
        **value** (*int*) -- The "int" value to be used with the
        command.

    Returns:
        The number of prime numbers less than or equal to "value".

    Return type:
        int

## rust_primes.func.list_primes(value: int, *, command: Union[CommandType, Literal['count', 'list', 'check']] = CommandType.LIST) -> Union[List[int], int]

    Return a list of all prime numbers less than or equal to "value".

    Parameters:
        **value** (*int*) -- The "int" value to be used with the
        command.

    Returns:
        A "list" of all prime numbers less than or equal to "value".

    Return type:
        List[int]

## rust_primes.func.is_prime(value: int, *, command: Union[CommandType, Literal['count', 'list', 'check']] = CommandType.CHECK) -> Union[List[int], int]

    Check if the provided "int" is a prime number.

    Parameters:
        **value** (*int*) -- The "int" value to be used with the
        command.

    Returns:
        "True" if the number is a prime number; otherwise "False".

    Return type:
        bool
