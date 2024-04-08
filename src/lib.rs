mod expr;
mod slate;

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
}