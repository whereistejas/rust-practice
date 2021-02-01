use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

fn simulated_algorithm(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random: u32) {
    // this is much better than the previous implementation, because we are only calliing
    // the expensive `simulated_algorithm` only once
    let intensity_val = simulated_algorithm(intensity);

    if intensity < 25 {
        println!("do pushups {} times", intensity_val);
        println!("do situps {} times", intensity_val);
    } else {
        if random == 3 {
            // but we dont need it, if this is going to be the end case
            println!("rest for a day")
        } else {
            println!("run around for {} minutes", intensity_val);
        }
    }
}

fn main() {
    let user_input = 50;

    let mut generator = thread_rng();
    let random_input = generator.gen_range(1..5);

    generate_workout(user_input, random_input);
}
