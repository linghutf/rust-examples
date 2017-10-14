
use std::env;
use std::str::FromStr;

fn fib(a:u64, b:u64, num:u64) -> u64{
    if num == 1 {
        return b;
    }
    return fib(b,a + b, num - 1)
}

fn myfib(num:u64) -> u64 {
    return fib(0, 1, num);
}

fn fib_not(num:u64) -> u64 {
    if num == 1 || num == 2{
        return 1;
    }
    return fib_not(num-1) + fib_not(num-2);
}

pub fn main() {
    println!("hello rust!");
    println!("rec:{}, not_rec:{}", myfib(12), fib_not(12));

    let mut vec = Vec::new();
    for args in env::args() {
        vec.push(args);
    }

    let num = u64::from_str(&vec[1]).expect("error argument!");
    println!("result : {}, {}", myfib(num), fib_not(num));
}
