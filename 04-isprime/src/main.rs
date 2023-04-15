fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n/2 { // Range is [0-n/2]
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    for n in 0..32 { // Range is [0-32)
        if is_prime(n) {
            println!("{} is prime.", n);
        } else {
            println!("{} is not prime.", n);
        }
    }
}
