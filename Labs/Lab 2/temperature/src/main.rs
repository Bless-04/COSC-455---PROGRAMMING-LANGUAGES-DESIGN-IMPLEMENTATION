fn main() {
    let celsius: f64 = 23.0;
    let fahrenheit: f64 = to_fahrenheit(celsius);


    assert_eq!(to_fahrenheit(0.0), 32.0); //freezing
    assert_eq!(fahrenheit, 73.4); //linkedin

    println!("Test Passed!");

}

fn to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 9.0 / 5.0 + 32.0
}
