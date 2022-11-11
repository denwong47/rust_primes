use crate::primes::list_primes;

/// Find the mathematical upper bound of the nth prime.
pub fn upper_bound_of_nth_prime(n:u64) -> u64 {
    match n {
        n if n >= 8009824 => upper_bound_of_nth_prime_korollar_g(n),
        n if n >= 688383 => upper_bound_of_nth_prime_dusart_2010(n),
        n if n >= 6 => upper_bound_of_nth_prime_rosser_schoenfeld_1962(n),
        5 => 11,
        4 => 7,
        3 => 5,
        2 => 3,
        1 => 2,
        _ => 0,
    }
}

fn upper_bound_of_nth_prime_korollar_g(n:u64) -> u64 {
    let nf = n as f64;

    return (
        nf * (
            nf.ln() + nf.ln().ln() - 1.
            + (nf.ln().ln() - 2.) / nf.ln()
            - (
                nf.ln().ln().powi(2) - 6. * nf.ln().ln() + 10.273
            ) / 2. / nf.ln().powi(2)
        )
    ) as u64
}

fn upper_bound_of_nth_prime_dusart_2010(n:u64) -> u64 {
    let nf = n as f64;

    return (
        nf * (nf.ln() + nf.ln().ln() - 1.
        + (nf.ln().ln() - 2.) / nf.ln())
    ) as u64
}

fn upper_bound_of_nth_prime_rosser_schoenfeld_1962(n:u64) -> u64 {
    let nf = n as f64;

    return (nf * (nf * nf.ln()).ln()) as u64;
}

/// List the first n number of primes.
pub fn list_n_primes(n:u64) -> Vec<u64> {
    let ubound = upper_bound_of_nth_prime(n);

    return list_primes(ubound, Some(n));
}

/// Return the nth prime.
pub fn nth_prime(n:u64) -> Option<u64> {
    // We have to dereference the in
    return match &list_n_primes(n).last(){
        &Some(&prime) => Some(prime),
        None => None,
    };
}