use std::io;

fn main() {
    println!("N: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    println!("n = {n}");
}
