use std::vec;

const CHINESE: [&str; 11] = [
    "ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi",
];

const ENGLISH: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

///returns number vector
/// works by splitting by word and matching it to number
fn go(input: Vec<&str>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    result
}

fn main() {
    println!("{:?}", go(vec!["yi", "three", "wu", "ten", "ling"]));
}
