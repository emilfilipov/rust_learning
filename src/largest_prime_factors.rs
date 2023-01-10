// Largest prime factor https://projecteuler.net/problem=3
pub fn prime_factors(mut n: i64) {
    /*
    Description:
    This function takes a number and finds its prime factors - prime numbers that divide the number whole
    until the result is 1.
    Prime numbers are numbers that can only be divided by themselves or 1.

    Parameters:
    n: Range<i64> This size is chosen to allow larger numbers to play around.

    Output:
    Prints out the chosen number's prime factors.
     */
    let mut nat_numb: i64 = 1; // The purpose of this variable is to iterate over the prime number formula
    let mut factors: Vec<i64> = Vec::new(); // The resulting Vector that will be filled with the prime factors
    let mut prime: i64 = 2; // First prime number to start iterating

    while n > 1 {
        match (prime, n % prime) {

            // For 2 and 3 we can iterate like this
            (2..=3, 0) => {
                    factors.push(prime);
                    n /= prime;
                    prime += 1;
            }
            // Apart from 2 and 3, the formula for calculating a prime number is 6n+1 and 6n-1, where n is a natural number
            _ => {
                for a in vec![-1, 1] {
                    prime = 6 * nat_numb + a;
                    if n % prime == 0 {
                        factors.push(prime);
                        n /= prime;
                    }
                }
                nat_numb += 1;
            }
        }
        }
    println!("Largest prime factor of {}: {:?}", n, factors.iter().max().unwrap());

}