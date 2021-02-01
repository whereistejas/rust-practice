use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

fn simulated_algorithm(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random: u32) {
    let closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("do pushups {} times", closure(intensity));
        println!("do situps {} times", closure(intensity));
    } else {
        if random == 3 {
            // but we dont need it, if this is going to be the end case
            println!("rest for a day")
        } else {
            println!("run around for {} minutes", closure(intensity));
        }
    }
}

fn main() {
    let user_input = 50;

    let mut generator = thread_rng();
    let random_input = generator.gen_range(1..5);

    generate_workout(user_input, random_input);
}
