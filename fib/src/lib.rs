/// return n th fibonacci number
pub fn nth_fibonacci(n: u32) -> Result<u32, &'static str> {
    if n <= 0 {
        return Err("Input must be a positive integer");
    }

    Ok(nth_fibonacci_unsafe(n))
}

fn nth_fibonacci_unsafe(n: u32) -> u32 {
    if n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    nth_fibonacci_unsafe(n - 1) + nth_fibonacci_unsafe(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_zero() {
        let result = match nth_fibonacci(0) {
            Ok(_fib_num) => "success",
            Err(error_message) => error_message,
        };

        println!("error message : {}", result);
        assert_ne!(result.to_string(), "success");
    }

    #[test]
    fn test_fib_first() {
        let result = nth_fibonacci(1).unwrap();
        assert_eq!(result, 0);
    }
}
