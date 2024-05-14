use crate::config::Config;

mod expr;
mod slate;
pub mod config;
pub mod error;
pub mod mahal;
mod out;
pub mod marge;
mod joidis;

pub fn run(config: Config) -> Result<(), error::Error> {
    match config {
        Config::Example => {
            example();
            Ok(())
        }
        Config::Mahal(config) => { mahal::mahal(config) }
        Config::Marge(config) => { marge::marge(config) }
    }
}
pub fn example() {
    let slate = slate::Slate::new();
    let x = slate.new_var_str("x");
    let y = slate.new_var_str("y");
    let two = slate.new_num(2);
    println!("x = {x}");
    println!("y = {y}");
    println!("{}", x + y);
    println!("{}", x - y);
    println!("{}", x * y);
    println!("{}", x / y);
    println!("{}", (x / y) / (y / x));
    println!("{}", two * x * x + y * y);
    println!("{}", two * x.pow(two) + two * x + two);
}