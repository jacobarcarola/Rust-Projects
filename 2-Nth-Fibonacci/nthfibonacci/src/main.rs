// Generate the nth Fibonacci number. Can only go up to 186 before overflowing.

use std::io;

fn main() {
    println!("nth Fibonacci Number Generator");

    println!("Which Fibonacci number would you like to generate?");
    let n: u128 = get_string_return_int();
    let fib_n: u128 = nth_fib(n);
    println!("The {n}th Fibonacci number is {fib_n}.");
}

fn get_string_return_int() -> u128 {
    let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

    let n_int: Result<u128, _> = n.trim().parse();
    match n_int {
        Ok(result) => {
            return result;
        }
        Err(_) => {
            return 0;
        }
    }
}

// Iterative approach
// Why not recursion? Each call to the function would be stored on
// the stack, making recursion more expensive than iteration, even
// if it looks nicer.
fn nth_fib(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    } else {
        let mut i = 1;
        let mut a = 1;
        let mut b = 1;
        let mut c = 0;
        while i < n - 1 {
            c = a + b;
            a = b;
            b = c;
            i = i + 1;
        }
        return c;
    }
}