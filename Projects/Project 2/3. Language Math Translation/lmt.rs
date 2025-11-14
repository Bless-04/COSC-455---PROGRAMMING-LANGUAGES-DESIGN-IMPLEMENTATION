use std::vec;

const CHINESE: [&str; 11] = [
    "ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi",
];

const ENGLISH: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

///returns number vector
/// input should be split by words
fn go(input: Vec<&str>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for word in input {
        if let Some(pos) = CHINESE.iter().position(|&s| s.eq_ignore_ascii_case(word)) {
            result.push(pos as i32);
        } else if let Some(pos) = ENGLISH.iter().position(|&s| s.eq_ignore_ascii_case(word)) {
            result.push(pos as i32);
        }
    }

    result
}

fn main() {
    println!("{:?}", go(vec!["yi", "threE", "wu", "ten", "ling"]));
}
