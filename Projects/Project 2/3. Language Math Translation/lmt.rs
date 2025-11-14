use std::vec;

const CHINESE: [&str; 11] = [
    "ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi",
];

const ENGLISH: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

/// exists to match word doc output
/// n+...n = sum
fn print_sum(input: &Vec<i32>) -> i32 {
    print!("Addition: ");
    let mut sum: i32 = 0;
    for (i, &num) in input.iter().enumerate() {
        sum += num;
        if i < input.len() - 1 {
            print!("{} + ", num);
        } else {
            println!("{} = {}", num, sum);
        }
    }
    sum
}

fn print_product(input: &Vec<i32>) -> i32 {
    print!("Multiplication: ");
    let mut product: i32 = 1;
    for (i, &num) in input.iter().enumerate() {
        product *= num;
        if i < input.len() - 1 {
            print!("{} * ", num);
        } else {
            println!("{} = {}", num, product);
        }
    }
    product
}

///returns number vector
/// input should be split by words
fn go(input: &Vec<&str>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for word in input {
        if let Some(pos) = CHINESE.iter().position(|&s| s.eq_ignore_ascii_case(word)) {
            result.push(pos as i32);
        } else if let Some(pos) = ENGLISH.iter().position(|&s| s.eq_ignore_ascii_case(word)) {
            result.push(pos as i32);
        }
    }

    println!("Translation: {:?}", result);

    print_sum(&result);

    print_product(&result);

    result
}

fn main() {
    /*
        let input = vec!["yi", "nine", "six", "ba"];
        go(&input);

    */

    let input = vec!["yi", "josh", "three", "si"];
    go(&input);

    /*
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];


    assert_eq!(
        numbers,
        go(&vec![
            "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi"
        ]),
        "Chinese is failing"
    );

    assert_eq!(
        numbers,
        go(&vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"
        ]),
        "English is failing"
    );
    */
}
