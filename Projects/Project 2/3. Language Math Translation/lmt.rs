use std::vec;

/// matches chinese or english word to number or none
fn match_word(input: &str) -> Option<i32> {
    match input.to_ascii_lowercase().as_str() {
        "ling" | "zero" => Some(0),
        "yi" | "one" => Some(1),
        "er" | "two" => Some(2),
        "san" | "three" => Some(3),
        "si" | "four" => Some(4),
        "wu" | "five" => Some(5),
        "liu" | "six" => Some(6),
        "qi" | "seven" => Some(7),
        "ba" | "eight" => Some(8),
        "jiu" | "nine" => Some(9),
        "shi" | "ten" => Some(10),
        _ => None,
    }
}

/// Exist to only print

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
/// exists because no looping
fn translation_helper(input: &Vec<&str>, result: &mut Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    if let Some(number) = match_word(input.first().unwrap()) {
        result.push(number);
    }

    translation_helper(&input[1..].to_vec(), result) //to_vec() fixed issue
}

///returns number vector
/// input should be split by words
fn go(input: &Vec<&str>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    let mut result: Vec<i32> = vec![];

    translation_helper(input, &mut result);

    println!("Translation: {:?}", result);

    print_sum(&result);

    print_product(&result);

    println!();
    result
}

fn main() {
    /*
        let input = vec!["yi", "nine", "six", "ba"];
        go(&input);

    */

    let input = vec!["yi", "josh", "three", "si"];
    go(&input);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(
        numbers,
        go(&vec![
            "skip", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi"
        ]),
        "Chinese is failing"
    );

    assert_eq!(
        numbers,
        go(&vec![
            "skip", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"
        ]),
        "English is failing"
    );
}
