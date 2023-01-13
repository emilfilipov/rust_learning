// 10001st prime https://projecteuler.net/problem=7
//use std::{f64};
use std::time::Instant;

pub fn get_prime() {
    /*
    The function prints out the 1
     */

    let start = Instant::now();

    let n: i64 = 10001;
    let mut primes: Vec<i64> = vec![2, 3];
    let mut natural_number: i64 = 1;


    let mut nprime: i64 = 2; // we start from prime #3 thus the index = 2
    while nprime < n {
        // Here we generate the pairs of prime potentials according to the formula 6n +/- 1
        // and iterate over them, checking each if it is a prime
        let primes_to_test: Vec<i64> = vec![6 * natural_number - 1, 6 * natural_number + 1];

        for prime_to_test in primes_to_test {
            // We calculate the square of the prime candidate
            let square: i64 = (prime_to_test as f64).sqrt() as i64;
            let mut is_prime: bool = true;

            for prime in &primes {
                // compare the prime candidate with each prime under its square root
                // if there is even one that divides it without remainder, break the loop
                if prime <= &square && prime_to_test % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            // if no primes divide the prime candidate without remainder
            // add it to the primes vector, increase the nprime counter and continue
            if is_prime == true {
                primes.push(prime_to_test);
                nprime += 1;
            }
        }
        // increase n in 6n +/- 1
        natural_number += 1;

    }

    let len = primes.len();
    println!("{:?}", len);
    println!("{:?}", primes[len-1]);
    let duration = start.elapsed();
    println!("{:?}", duration);

}