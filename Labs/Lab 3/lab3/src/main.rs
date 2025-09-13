fn main() {
    let test1 = "ABC123DEF".to_string();
    let test2 = "123ABC".to_string();

    // Assertions for the test cases
    assert_eq!(q1_parser(test1), true);
    assert_eq!(q1_parser(test2), false);
}

// Utility method that takes a character and returns true if digit
// or upper case letter.
fn is_uppercase_or_digit(c: char) -> bool {
    is_uppercase_letter(c) || (c >= '0' && c <= '9')
}

// Utility method that takes a character and returns true if it is
// an upper case letter.
fn is_uppercase_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

/* language	: UPPERCASE_LETTER UPPERCASE_LETTER rest?; */
fn q1_parser(text: String) -> bool {

    // make String into char vector
    let characters_array: Vec<char> = text.chars().collect();

    // for each character in the character vector do...
    // where i starts at 1 and increments for each iteration
    for (i, character) in characters_array.iter().enumerate() {

        //checks first two inputs for uppercase letter
        if i <= 1 && !is_uppercase_letter(*character) {
            return false;
        }

        //checks if {rest?} is uppercase or digit;
        // tests pass without this; maybe an easter egg
        else if !is_uppercase_or_digit(*character) {
            return false;
        }

    }
    return true;
}
