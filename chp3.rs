use rand::Rng;

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("x val is {}", x);
    another_function(5);
    println!("plus_one(5) is {}", plus_one(5));

    println!("temp is {}", c_to_f(20.0));

    for n in 0..=9{
        println!("{}: {}", n, fib(n));
    }
}

fn another_function(x: i32) {
    println!("another function {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn c_to_f(temperature: f64) -> f64 {
    (temperature * 9.0 / 5.0) + 32.0
}

fn fib(n: u32) -> u64 {
    let mut first: u64 = 0;
    let mut second: u64 = 1;

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    for _ in 2..=n {
        let current = first + second;
        first = second;
        second = current;
    }

    second
}
