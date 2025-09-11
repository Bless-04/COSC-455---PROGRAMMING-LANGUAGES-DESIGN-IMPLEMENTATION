fn main() {

    let a: f64 = 13.0;
    let b: f64 = 2.3;
    let c: f64 = 120.0;

    let average: f64 = (a + b + c) / 3.0;

    assert_eq!(average, 45.1);
    println!("Average works");

}
