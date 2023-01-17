// Special Pythagorean triplet https://projecteuler.net/problem=9
use std::time::Instant;

pub fn pyth_triplets() {

    // This is essentially brute forcing
    let start = Instant::now();
    'outer: for a in 1..=1000 as i32 {
        for b in 1..=(1000 - a) as i32 {
            let c = 1000 - a - b;
            if (a.pow(2) + b.pow(2)) == c.pow(2) {
                println!("{} {} {} ---- {}", a, b, c, a*b*c);
                break 'outer;
            }
        }
    }
    let duration = start.elapsed();
    println!("{:?}", duration);

    // m, n formula
    let start = Instant::now();
    'outer: for m in 1..50 as i32 {
        for n in 1..m as i32 {
            let a: i32 = m.pow(2) - n.pow(2);
            let b: i32 = 2 * m * n;
            let c: i32 = m.pow(2) + n.pow(2);

            if a + b + c == 1000 {
                println!("{} {} {} ---- {}", a, b, c, a*b*c);
                break 'outer;
            }
        }
    }
    let duration = start.elapsed();
    println!("{:?}", duration);

    // Brute force:     4.976ms
    // M, N formula:    7.8µs

}