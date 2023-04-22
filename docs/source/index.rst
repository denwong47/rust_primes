Not too terrible Prime Calculator
=================================

\...written in Rust with Python bindings.

This is mostly a personal project to learn Rust. It is more of a
study of parallelism in Rust, and the use of `pyo3 <https://docs.rs/pyo3/latest/pyo3/>`_
to export Python bindings in Rust code.

The subject of Prime sieving was chosen since Python does not have a true implementation
of multi-threading due to the use of
`GIL <https://wiki.python.org/moin/GlobalInterpreterLock>`_, while its flexibility in
typing enables convenience in working with large datasets of unknown quality.
Traditionally most compute heavy tasks in Python are written in C libraries instead, but
memory safety issues are common in such low level implementation, causing Segementation
Faults obscured behind the CPython making it impossible to troubleshoot.

Rust provides the memory safety as well as a modern set of tools to work with true
concurrency, making it the perfect partner with Python.

See `Rust Docs <./lib/doc/lib_rust_primes/index.html>`_ for Rust bindings.

Sample output and runtime on an Apple M1 Pro:

.. code-block:: text

   Calculating all primes under 10^4:
   - SieveMethod.ATKIN                       : 0.000024s
   - SieveMethod.ERATOSTHENES                : 0.000011s
   - SieveMethod.ERATOSTHENES_THREADED       : 0.000142s
   - SieveMethod.ERATOSTHENES_ATOMIC         : 0.000115s

   Calculating all primes under 10^5:
   - SieveMethod.ATKIN                       : 0.000190s
   - SieveMethod.ERATOSTHENES                : 0.000099s
   - SieveMethod.ERATOSTHENES_THREADED       : 0.000221s
   - SieveMethod.ERATOSTHENES_ATOMIC         : 0.000288s

   Calculating all primes under 10^6:
   - SieveMethod.ATKIN                       : 0.001648s
   - SieveMethod.ERATOSTHENES                : 0.001109s
   - SieveMethod.ERATOSTHENES_THREADED       : 0.001364s
   - SieveMethod.ERATOSTHENES_ATOMIC         : 0.001368s

   Calculating all primes under 10^7:
   - SieveMethod.ATKIN                       : 0.014422s
   - SieveMethod.ERATOSTHENES                : 0.011521s
   - SieveMethod.ERATOSTHENES_THREADED       : 0.024931s
   - SieveMethod.ERATOSTHENES_ATOMIC         : 0.011437s

   Calculating all primes under 10^8:
   - SieveMethod.ATKIN                       : 0.447571s
   - SieveMethod.ERATOSTHENES                : 0.423881s
   - SieveMethod.ERATOSTHENES_THREADED       : 0.222338s
   - SieveMethod.ERATOSTHENES_ATOMIC         : 0.153570s

   Calculating all primes under 10^9:
   - SieveMethod.ATKIN                       : 5.407987s
   - SieveMethod.ERATOSTHENES                : 5.830421s
   - SieveMethod.ERATOSTHENES_THREADED       : 2.663690s
   - SieveMethod.ERATOSTHENES_ATOMIC         : 2.096283s

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   rust_primes


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
