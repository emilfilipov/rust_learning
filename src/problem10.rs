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



    // Sieve of Eratosthenes method
    let start = Instant::now();

    // We generate a vector of true values that we will evaluate
    let limit = 2000000;
    let mut primes = vec![true; limit];
    let mut prime = 2;

    // Each true of the vector represents a number that is its index + 1
    // So index 0 represents the number 1
    // Since 1 is not a prime, we can just set it to false
    primes[0] = false;

    loop {
        // Next, we will iterate over the primes vector with p and remove its multiples
        let mut multiple_index = 2 * prime - 1;
        // 2 * 2 - 1 = 3, index 3 = 4, since 2 is the first known prime
        // We iterate over all multiples and set them to false
        while multiple_index < limit {
            primes[multiple_index] = false;
            multiple_index += prime; // each time we increase i with the prime until it becomes larger than our limit
            // i = 3, then i += prime ... i = 3 + 2
            // index 5 = 6, then i = 5 + 2 = 7, index 7 = 8 and so on until all multiples of 2 are removed
        }

        // With the next if statement we look for the next index where there is a true value
        // This is why we search a range from the current prime to the limit
        // Since we have already removed all multiple of 2 (from iteration 1)
        // This statement should give us indexes of numbers that represent numbers above 2
        if let Some(n) = (prime..limit).find(|&n| primes[n] == true) {
            // Find iterates over the range and gives us the first n (so index) where the value is true
            // Let's print out what we find just in case
            //println!("Current prime {}... Next prime {} found on index {}", prime, n+1, n);
            // If the find statement finds another index that is set to true in the primes vector
            // Remember that each index represents a number = index + 1
            // [2,3,4,5,6,7...] indexes we search for
            // [1,2,3,4,5,6...] index 2 in primes = the number 3, which is still true since it's not a multiple of 2
            // We set the next prime number to remove multiples of to that index + 1
            prime = n + 1;
        } else {
            break;
        }
    }

    // Once we are out of new true values in the primes vector, we simply have to iterate over it and calculate the sum of all elements
    let result: Vec<usize> = primes
                    .iter() // iterate over
                    .enumerate() // show the indexes
                    .filter(|&(_, &is_prime)| is_prime == true) // remove all values that are false
                    .map(|(i, _)| i + 1) // convert the indexes to the represented number, so index + 1
                    .collect();
    let sum: usize = result.iter().sum();

    println!("{:?}", sum); // iterate and find the sum
    let duration = start.elapsed();
    println!("{:?}", duration);

}















