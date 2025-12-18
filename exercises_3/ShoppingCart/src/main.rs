mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use crate::container::Container;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("test"));

    let mut s1 = Stack::new(vec![String::from("test")]);

    add_string(&mut b1, String::from("test"));
    add_string(&mut s1, String::from("test"));
}
