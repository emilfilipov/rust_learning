// Sum square difference https://projecteuler.net/problem=6
use std::ops::{Range, RangeInclusive};

fn square_of_sum(r: RangeInclusive<i32>) -> i32 {
    let sum: i32 = r.sum();

    return sum * sum;
}

fn sum_of_squares(r: RangeInclusive<i32>) -> i32 {
    let mut squares: Vec<i32> = Vec::new();

    for i in r.clone() {
        squares.push(i*i);
    }

    return squares.iter().sum()
}

pub fn sum_square_difference() {
    let range1: RangeInclusive<i32> = 1..=100;
    let range2: RangeInclusive<i32> = 1..=100;

    let square_of_sum: i32 = square_of_sum(range1);
    let sum_of_squares: i32 = sum_of_squares(range2);

    println!("{}", square_of_sum - sum_of_squares);
}