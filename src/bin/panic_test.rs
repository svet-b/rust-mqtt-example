use std::{time, thread, panic};

fn main() {
    while thread::spawn(move || panic!()).join().is_err() {
        println!("Respawn!");
        thread::sleep(time::Duration::from_secs(3));
    }

    // this line won't ever be invoked because of process::exit()
    println!("Finished");
}
