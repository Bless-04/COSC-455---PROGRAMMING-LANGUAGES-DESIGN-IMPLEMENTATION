// Functional Programming Examples in Rust
// Demonstrating pattern matching, recursion, and map-style higher-order functions.

// power function to raise number to a power
pub fn pow(base: f64, exp: i32) -> f64 {
    match exp {
        1 => base,                      // when its 1
        _ => base * pow(base, exp - 1), // for everything else
    }
}

// collatz sequence generating function
pub fn collatz(n: i32) -> Vec<i32> {
    match n {
        1 => vec![1],
        _ if n % 2 == 0 => {
            let mut seq = vec![n];
            seq.extend(collatz(n / 2));
            seq
        }
        _ => {
            let mut seq = vec![n];
            seq.extend(collatz(3 * n + 1));
            seq
        }
    }
}

// makes a twin of things in a list
pub fn twins(xs: &[i32]) -> Vec<i32> {
    match xs {
        [] => Vec::new(),
        [x, rest @ ..] => {
            let mut front = vec![*x, *x];
            front.extend(twins(rest));
            front
        }
    }
}

// turns true to 1 and false to 0 using vec
pub fn convert(xs: &[bool]) -> Vec<i32> {
    xs.iter()
        .map(|&b| match b {
            true => 1,
            false => 0,
        })
        .collect()
}

// turns int 32 to a decimal number
fn to_real(x: i32) -> f64 {
    x as f64
}

pub fn il2rl(xs: &[i32]) -> Vec<f64> {
    xs.iter().copied().map(to_real).collect()
}

// integer^2
fn square(x: i32) -> i32 {
    x * x
}

// same as square but for lists
pub fn squarelist(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().map(square).collect()
}

fn main() {
    // pow
    println!("pow(2.0, 3) = {}", pow(2.0, 3)); // 8.0
    println!("pow(1.5, 4) = {}", pow(1.5, 4)); // 5.0625
    println!();

    // collatz
    println!("collatz(6) = {:?}", collatz(6)); // [6, 3, 10, 5, 16, 8, 4, 2, 1]
    println!();

    // twins
    println!("twins([1,2,3])       = {:?}", twins(&[1, 2, 3]));
    println!("twins([])            = {:?}", twins(&[]));
    println!();

    // convert
    println!(
        "convert([true,true,false,true]) = {:?}",
        convert(&[true, true, false, true])
    );
    println!();

    // il2rl
    println!("il2rl([1,2,3])       = {:?}", il2rl(&[1, 2, 3]));
    println!();

    // squarelist
    println!("squarelist([1,2,3,4]) = {:?}", squarelist(&[1, 2, 3, 4]));
}
