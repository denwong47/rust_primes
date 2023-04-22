use ndarray::OwnedRepr;

use super::{CanBeTrue, OwnedSieve};

#[allow(unused_imports)]
use ndarray::Array;

/// Trait allowing [`Array`]s of [`bool`] can be collected into
/// [`Vec<u64>`] after sieving.
pub trait SievedArray<T, E>
where
    T: ndarray::Data<Elem = E>,
    E: CanBeTrue,
{
    fn is_prime_index(value: &E) -> bool;
    fn count_primes(&self) -> u64;
    fn collect_into_primes(&self, n_limit: Option<u64>) -> Vec<u64>;
}
impl<E> SievedArray<OwnedRepr<E>, E> for OwnedSieve<E>
where
    E: CanBeTrue,
{
    fn is_prime_index(value: &E) -> bool {
        value.is_true()
    }

    fn count_primes(&self) -> u64 {
        self.iter()
            .filter(|&value| Self::is_prime_index(value))
            .count() as u64
    }

    fn collect_into_primes(&self, n_limit: Option<u64>) -> Vec<u64> {
        let result = self
            .iter()
            .enumerate()
            .filter(|&(_, value)| Self::is_prime_index(&value))
            .map(|(index, _)| index as u64);

        return if let Some(n) = n_limit {
            result.take(n as usize).collect::<Vec<u64>>()
        } else {
            result.collect::<Vec<u64>>()
        };
    }
}
