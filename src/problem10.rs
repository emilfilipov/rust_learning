// Summation of primes https://projecteuler.net/problem=10
use std::time::Instant;

fn is_prime(prime_to_test: i64) -> bool {
    if prime_to_test == 1 {
        return false;
    }
    if prime_to_test == 2 {
        return true;
    }
    let square: i64 = (prime_to_test as f64).sqrt() as i64;

    for n in 2..=square {
        if prime_to_test % n == 0 {
            return false;
        }
    }
    return true;
}

pub fn summation_of_primes() {

    // Bruteforce method
    let start = Instant::now();

    let limit: i64 = 2000000;
    let mut rest: Vec<i64> = vec![2];

    // start with 3 and a step of 2 as even numbers can never be primes by default
    for n in (3..limit).step_by(2) {
        if is_prime(n) == true {
            rest.push(n);
        }
    }

    let result: i64 = rest.iter().sum();
    println!("{:?}", result);
    let duration = start.elapsed();
    println!("{:?}", duration);

}