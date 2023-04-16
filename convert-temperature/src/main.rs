use std::io;
fn main() {
    let mut fahrenheit = String::new();
    let mut num: f32 = 0.0;

    println!("Please enter value in fahrenheit");
    io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Failed to read fahrenheit");

    num = fahrenheit.trim().parse().expect("Not a valid string");

    let ans = (num - 32.0) / 1.8;

    println!("Converted value from fahrenheit to celsius {:.2}", ans);
}
