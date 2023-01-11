// Smallest Multiple https://projecteuler.net/problem=5

fn generate_primes(limit: i64) -> Vec<i64> {
    /*
    Helper function to generate prime numbers in a given range.
     */
    let mut primes: Vec<i64> = vec![2,3];
    let mut nat_numb: i64 = 1;

    while 6 * nat_numb + 1 < limit+1 {
        primes.push(6 * nat_numb - 1);
        primes.push(6 * nat_numb + 1);
        nat_numb += 1;
    }
    return primes;
}

pub fn smallest_multiple_range(limit: i64) {
    /*
    The function returns the lowest common multiple of a range of numbers.
     */
    //

    let primes = generate_primes(limit); // generate primes up to the limit
    let mut summarize_this = primes.clone(); // clone the primes in the result vector
    let mut values: Vec<i64> = Vec::new(); // init an empty vector to host numbers that are not in primes

    // Fill values vector with numbers that are not in the primes vector
    for n in 2..=limit {
        if primes.contains(&n) == false {
            values.push(n);
        }
    }

    // Since we have already separated the primes from the non-prime values
    // It makes sense to just make all the necessary divisions
    for prime in &primes {
        for ind in 0..values.len() {
            if values[ind] % prime == 0 {
                values[ind] = values[ind] / prime;
            }
        }
    }
    values.retain(|&x| x != 1);

    // We will keep iterating over the leftover values in the values Vector
    // Each iteration we will make the same division as above with the addition
    // Of checking if we have already added the prime divisor in the iteration
    // This is to avoid adding the prime value more than once
    // At the end of each iteration we will remove all elements that are == 1
    // This should, provide a speed up since we iterate over a smaller vector with each iteration
    while values.len() > 0 {

        for prime in &primes {
            values.retain(|&x| x != 1);
            let mut iter_val: i64 = 0;

            for ind in 0..values.len() {

                if values[ind] % prime == 0 {

                    values[ind] = values[ind] / prime;

                    if iter_val == 0 {

                        summarize_this.push(*prime);
                        iter_val = 1;
                    }
                }
            }
        }
    }

    println!(" ");
    println!("RESULT LIST   {:?}", summarize_this);
    let mut product: i64 = 1;
    for i in summarize_this {
        product *= i;
    }
    println!("PRODUCT       {}", product);

}