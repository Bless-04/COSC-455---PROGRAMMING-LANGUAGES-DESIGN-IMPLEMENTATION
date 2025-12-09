// Helper function to check prime
fn is_primeh(n: i32, d: i32) -> bool {
    if d * d > n {
        return true;
    }
    if n % d == 0 {
        return false;
    }
    is_primeh(n, d + 1)
}

// true if prime
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    is_primeh(n, 2)
}

// finds pair of primes
fn find_pair(n: i32, i: i32) {
    if i > n / 2 {
        return; // shouldnt happen
    }

    let complement = n - i;

    if is_prime(i) && is_prime(complement) {
        println!("{} + {} = {}", i, complement, n);
    } else {
        find_pair(n, i + 1);
    }
}

fn goldbach(n: i32) {
    // Error checking
    if n <= 2 || n % 2 != 0 {
        println!("Error: Input must be an even integer greater than 2.");
        return;
    }

    // Start recursion from 2
    find_pair(n, 2);
}

fn main() {
    goldbach(28); // Expected: 5 + 23 = 28
    goldbach(100); // Expected: 3 + 97 = 100
    goldbach(4); // Expected: 2 + 2 = 4
    goldbach(11); // Expected: Error message
}
