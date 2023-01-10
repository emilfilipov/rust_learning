// Largest palindrome product https://projecteuler.net/problem=4
fn is_palindrome(i: i32) -> bool {
    /*
    Helper function to check if an integer is a palindrome.
     */
    let numstr: String = i.to_string();
    let reversed = numstr.chars().rev().collect::<String>();

    return numstr == reversed
}

pub fn largest_palindrome_product() {
    /*
    This function returns the largest palindrome product of 3-digit numbers.
     */
    // Initialize variables to carry the process
    let mut largest: i32 = 0;
    let mut product: i32;
    // Since we are looking for the largest palindrome product, it makes sense
    // to reverse the ranges and start with the largest numbers
    for n in (100..=999).rev() {
        for m in (100..=999).rev(){
            product = n * m;
            // If the product is larger than the current larger variable, we carry over the value
            // And break the inner loop as, logically, the subsequent products will be smaller
            if product > largest && is_palindrome(product) == true {
                largest = product;
                break
            }
            // If the current largest palindrome is larger than the product, then break the loop
            else if largest > product {
                break
            }
        }
    }
    println!("Largest palindrome product of 2 3-digit numbers: {}", largest);
}