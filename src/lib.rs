use crate::scalar::var;

mod scalar;

pub fn example() {
    let x = var('x');
    let y = var('y');
    println!("x = {x}");
    println!("y = {y}");
}