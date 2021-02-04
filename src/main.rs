use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        let v = self.value.entry(arg).or_insert((self.calculation)(arg));
        *v
    }

}

fn generate_workout(intensity: u32, random: u32) {
    let mut closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("do pushups {} times", closure.value(intensity));
        println!("do situps {} times", closure.value(intensity));
    } else {
        if random == 3 {
            // but we dont need it, if this is going to be the end case
            println!("rest for a day")
        } else {
            println!("run around for {} minutes", closure.value(intensity));
        }
    }
}

fn main() {
    let user_input = 50;

    let mut generator = thread_rng();
    let random_input = generator.gen_range(1..5);

    generate_workout(user_input, random_input);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
