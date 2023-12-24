use std::io;

fn main() {
    println!("n: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n
        .trim()
        .parse()
        .expect("Please input a number");

    println!("n * 2 = {}", n * 2);
}
