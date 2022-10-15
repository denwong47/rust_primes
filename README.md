#### *behold...*
# YOU ARE LOOKING AT A **MIRACLE** OF PROGRAMMING.
## Witness history in the making.
### The impossible had been achieved.

<br /><br /><br />


#### ...Some idiot managed to make `Rust` slower than `Python`.

<!-- ![This guy sucks](https://i.kym-cdn.com/entries/icons/mobile/000/000/554/picard-facepalm.jpg) -->
# UPDATE: Well, not anymore!

![Thanks Ted](./6r1atj.jpg)

Problem solved!

## Usage:

```sh
rust_primes check 79        # Check if a number is prime
rust_primes count 1000      # Count the number of primes <= 1000
rust_primes list 1000       # List all the primes <=1000 in [2, 3, 5... ] format

# For use as JSON
rust_primes list 1000 raw    # Remove all readable strings and print result only.

# For use with Python
rust_primes list 1000 pickle # Remove all readable strings and print result as Python compatible pickled data.
```

------

Well as the name of this repo suggests, I am learning Rust.

And I wrote a classic prime number generator, only to find that it is slower than my `Python numba` implementation.
Not only that, the `Rust` dumb for-loop beats the threaded version of it by staggering margins.

## *Send help?*