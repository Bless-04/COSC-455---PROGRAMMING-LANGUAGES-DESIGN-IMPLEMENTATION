struct Circle {
    radius: f64, // assume radius > 0
}

impl Circle {
    const PI: f64 = 3.14; // Pi constant as f64 for no math problems

    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        Circle::PI * (self.radius * self.radius)
    }

    fn circumference(&self) -> f64 {
        2.0 * Circle::PI * self.radius
    }
}

fn main() {
    let test: f64 = 1.0;
    let circle = Circle::new(test);

    println!("Area: {}", circle.area());
    println!("Circumference: {}", circle.circumference());

    assert!(Circle::PI == circle.area());
    assert!(2.0 * Circle::PI == circle.circumference());
}
