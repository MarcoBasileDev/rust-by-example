use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.imag < 0.0 { '-' } else { '+' };
        write!(f, "{} {}{}i", self.real, sign, self.imag.abs())
    }
}

fn main() {

    let c1 = Complex { real: 3.3, imag: 7.2 };
    let c2 = Complex { real: 4.7, imag: -2.3 };

    println!();
    println!("Display: {}", c1);
    println!("Debug: {:?}", c1);

    println!();
    println!("Display: {}", c2);
    println!("Debug: {:?}", c2);

}