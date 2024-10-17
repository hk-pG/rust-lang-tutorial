use fib::nth_fibonacci;
use std::io;

fn main() {
    // read user input 'number'
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    // expected unsigned number
    let number = buf.trim().parse::<u32>().unwrap();

    let result = match nth_fibonacci(number) {
        Ok(fib_num) => fib_num,
        Err(error_message) => {
            panic!("{}", error_message);
        }
    };

    println!("nth fibonacci number: {}", result);
}
