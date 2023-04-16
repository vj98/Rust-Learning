use std::io;
fn main() {
    println!("Please enter number for nth fibonacci number");

    let mut n: String = String::new();
    let mut num: usize = 0;
    io::stdin().read_line(&mut n).expect("Not a valid string");
    
    num = n.trim().parse().expect("Not a valid number");
    let mut arr = Vec::new();
    arr.push(0);
    arr.push(1);
    arr.push(1);

    println!("-------------------------------");
    for i in 3..num+1 {
        arr.push(arr[i-1] + arr[i-2]);
    }

    println!("nth fibonacci is {}", arr[num]);
}
