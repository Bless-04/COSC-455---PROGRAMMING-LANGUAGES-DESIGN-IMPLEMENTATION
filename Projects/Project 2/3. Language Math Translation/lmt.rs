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

/// n+...n = sum
fn print_sum(input: &Vec<i32>, index: usize, mut sum: i32) -> i32 {
    sum += input[index];
    if index == input.len() - 1 {
        println!("{} = {}", input[index], sum);
        return sum;
    }
    print!("{} + ", input[index]);
    print_sum(input, index + 1, sum)
}

fn print_product(input: &Vec<i32>) -> i32 {
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

    print!("Addition: ");
    print_sum(&result, 0, 0);

    print!("Multiplication: ");
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
