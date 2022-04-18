use primal::Sieve;

type Factors = Vec<(usize, usize)>;

pub fn sieve(num: usize) -> bool {
    let sieve = Sieve::new(10000);
    sieve.is_prime(num)
}

pub fn nth(num: usize) -> Option<usize> {
    let sieve = Sieve::new(10000);
    sieve.primes_from(0).nth(num - 1)
}

pub fn factor(num: usize) -> Result<Factors, (usize, Factors)> {
    let sieve = Sieve::new(10000);
    sieve.factor(num)
}

pub fn count_divisors(num: usize) -> Option<usize> {
    let sieve = Sieve::new(10000);
    match sieve.factor(num) {
        Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1))),
        Err(_) => None,
    }
}