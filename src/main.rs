mod math;

fn main() {
    // let mut _obj : &[u64];

    // _obj = &(2..30).collect::<Vec<u64>>();
    println!("{:?}", math::is_prime(79, None));
    println!("{:?}", math::list_primes(1000000).len());
}
