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

/// vec apiの使い方のおべんきょう
mod learn_vec {
    /// シンプルなインデックスアクセス
    pub fn basic_indexes() {
        let vec = [1, 2, 3];
        // print using index accesses
        println!("{}, {}, {}", vec[0], vec[1], vec[2]);
    }

    /// イテレータと集計関数
    pub fn iterate_collect() {
        let vec = vec![4, 5, 6];

        let pow_vec: Vec<u32> = vec.into_iter().map(|x| x * x).collect();

        println!("{:?}", pow_vec);
    }

    pub fn iterate_loop() {
        let vec = [7, 8, 9];
        for elem in vec.iter() {
            println!("{}", elem);
        }
    }

    pub fn use_slice() {
        let vec = [10, 11, 12, 13, 14, 15];
        let slice = &vec[1..4];

        println!("vec            : {:?}", vec);
        println!("slice[1..4]    : {:?}", slice);
    }

    #[test]
    pub fn test_vec() {
        basic_indexes();
    }

    #[test]
    pub fn test_vec_collect() {
        iterate_collect();
    }

    #[test]
    pub fn test_vec_loop() {
        iterate_loop();
    }

    #[test]
    pub fn test_vec_slice() {
        use_slice();
    }
}
