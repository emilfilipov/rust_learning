// Multiples of 3 and 5 https://projecteuler.net/problem=1
use std::ops::Range;

pub fn multiples_of_num(r: Range<i32>) {
    /*
    Description:
    This function takes in a Range of numbers and gathers all elements that are multiples of 3 or 5.

    Parameters:
    r: Range<i32>

    Output:
    None
     */
    let mut num_sum: i32 = 0;
    let mut numbers = Vec::new();

    for number in r {
        if (number % 3 == 0) || (number % 5 == 0) {
            numbers.push(number);
            num_sum += number
        }
    }
    println!("Sum of multiples of 3 and 5: {}", num_sum);

}