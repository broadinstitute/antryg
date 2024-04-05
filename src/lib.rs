use crate::expr::var;

mod scalar;
mod expr;
mod slate;

pub fn example() {
    let x = var("x");
    let y = var("y");
    println!("x = {x}");
    println!("y = {y}");
}