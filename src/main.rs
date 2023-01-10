mod largest_palindrome_product;
mod even_fibonacci_sum;
mod largest_prime_factors;
mod multiples_of;

use crate::multiples_of::multiples_of_num;
use crate::largest_prime_factors::prime_factors;
use crate::even_fibonacci_sum::fib_seq;
use crate::largest_palindrome_product::largest_palindrome_product;

fn main() {
    // Problem 1
    println!("Solution to Problem 1!");
    multiples_of_num(47..1000);
    // Problem 2
    println!("Solution to Problem 2!");
    fib_seq(4000000);
    // Problem 3
    println!("Solution to Problem 3!");
    prime_factors(600851475143);
    // Problem 4
    println!("Solution to Problem 4!");
    largest_palindrome_product();
}
