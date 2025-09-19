struct Circle {
    radius: f64, // assume radius > 0
}

impl Circle {
    const PI: f64 = 3.14;

    fn new(radius: f64) -> Circle {
        Circle { radius}
    }

    fn area(&self) -> f64 {
        return Circle::PI * (self.radius * self.radius);
    }

    fn circumference(&self) -> f64 {
        return 2.0 * Circle::PI* self.radius;
    }

}


fn main(){
    let circle = Circle::new(5.0);

    println!("Area: {}", circle.area());
    println!("Circumference: {}", circle.circumference());
    assert_eq!(circle.area(), 78.5);
    assert_eq!(circle.circumference(), 31.400000000000002);
}