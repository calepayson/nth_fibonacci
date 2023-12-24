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
        .expect("Please input a positive number");

    let x = calculate_nth_fibonacci(n);

    println!("x = {x}");
}

fn calculate_nth_fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 || n == 2 {
        1
    } else {
        let mut current = 1;
        let mut previous = 1;

        let mut count = 0;
        loop {
            if count == (n - 2){
                break
            } else {
                let memory = current;
                current = current + previous;
                previous = memory;

                count += 1;
            }
        }
        current
    }
}
