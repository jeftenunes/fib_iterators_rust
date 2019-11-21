#[derive(Clone, Debug)]
struct Fibonacci {
    fib_1: i32,
    fib_n: i32,
    limit: Option<i32>,
}

impl Fibonacci {
    fn new(limit: Option<i32>) -> Fibonacci {
        Fibonacci {
            fib_1: 0,
            fib_n: 1,
            limit: limit, 
        }
    } 
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item>{
        let next_fib = self.fib_n + self.fib_1;
        println!("{:?}", next_fib);
        self.fib_1 = self.fib_n;
        self.fib_n = next_fib;

        if self.limit.is_none() {
            Some(next_fib)
        } else {
            if next_fib > self.limit.unwrap() {
                None
            } else {
                Some(next_fib)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test() {
        let fib = Fibonacci::new(None);
        let first_values: Vec<_> = fib.take(3).collect();
        assert_eq!(first_values, vec![1, 2, 3]);
    }
}