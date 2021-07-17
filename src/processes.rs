use std::time;
// use std::thread::sleep;
use std::process::{Command, *};

fn main() {
    // let five_second = time::Duration::from_secs(5);

    for i in 1..6 {
        let start_time = time::Instant::now();

        let mut command = Command::new("sleep")
            .arg("5")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to start echo child process");

        if i == 3 {
            loop {
                if ( time::Instant::now() - start_time ) > time::Duration::from_secs(3) {
                    command.kill()
                        .expect("failed to kill child process");
                    break;
                }
            }
        }

        let command_output = command.wait_with_output().expect("failed while waiting for echo output");
        
        println!("{}: {:?}", i, command_output);
        // sleep(five_second);
    }
}
