fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    // Increase the recursion limit to allow for larger values of n
    // Note: this is not necessary for small values of n
    //std::env::set_var("RUST_RECURSION_LIMIT", "10000");

    let n = 1;

    let result = fibonacci(n);
    println!("Function: The {}th Fibonacci number is: {}", n, result);
}
