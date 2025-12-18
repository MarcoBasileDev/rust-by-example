use num_traits::{Float, ToPrimitive};

// First version - we can pass in BOTH f32 or f64
fn solve_first_version<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}



fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;

    println!("{}", solve_first_version(a, b));
}
