#![allow(dead_code, unused_mut)]
use h_structs::Polygon;

fn main() {
    let mut polygon = Polygon::new("George".to_string());

    println!(
        "I see a {}-sided polygon named {}!",
        polygon.get_sides(), polygon.name
    );

    println!(
        "The polygon named {} is a {}",
        polygon.name,
        polygon.shape()
    );

    for _ in 0..3 {
        polygon.increment_sides();
        println!(
            "The polygon now has {} sides and is the shape of a {}",
            polygon.get_sides(),
            polygon.shape()
        );
    }
}