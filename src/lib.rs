mod expr;
mod slate;

pub fn example() {
    let slate = slate::Slate::new();
    let x = slate.new_var_str("x");
    let y = slate.new_var_str("y");
    let two = slate.new_num(2);
    let matrix2 = slate.new_matrix_fill(2, 2, |i, j| {
        if i == 0 {
            x
        } else {
            y
        }
    });
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