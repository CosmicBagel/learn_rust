use std::{hash::Hash, thread};
use std::time::Duration;
use std::{collections::HashMap};

struct Cacher<U, V>
where
    U: Copy + Eq + Hash,
    V: Copy + Eq,
{
    calculation: fn(U) -> V,
    hash_cache: HashMap<U, V>,
}

impl<U, V> Cacher<U, V>
where
    U: Copy + Eq + Hash,
    V: Copy + Eq,
{
    fn new(calculation: fn(U) -> V) -> Cacher<U, V> {
        Cacher {
            calculation,
            hash_cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.hash_cache.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.hash_cache.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} push-ups!", expensive_result.value(intensity));
        println!("Next, do {} sit-ups!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
