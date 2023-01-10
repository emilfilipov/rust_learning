// Even Fibonacci Numbers https://projecteuler.net/problem=2
pub fn fib_seq(limit: i32) {
    let mut fib_cur: i32 = 0;
    let mut fib_next: i32 = 1;
    let mut fib_new_next: i32;
    let mut even: Vec<i32> = Vec::new();

    while fib_cur < limit {

        // Check if current is even
        if fib_cur % 2 == 0 {
            even.push(fib_cur);
        }
        // Calculate next elements
        fib_new_next = fib_cur + fib_next;
        fib_cur = fib_next;
        fib_next = fib_new_next;

    }
    // Calculate Sum of even fibs
    let result: i32 = even.iter().sum();
    println!("Sum of all even Fibonacci elements under {}: {}", limit, result);
}