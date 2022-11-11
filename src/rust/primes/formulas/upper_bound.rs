
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
    0
}

fn upper_bound_of_nth_prime_dusart_2010(n:u64) -> u64 {
    0
}

fn upper_bound_of_nth_prime_rosser_schoenfeld_1962(n:u64) -> u64 {
    let nf = n as f64;

    return (nf * (nf * nf.ln()).ln()) as u64;
}
