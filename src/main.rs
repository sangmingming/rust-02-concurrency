use std::{
    thread::{self, sleep},
    time::Duration,
};
fn main() {
    println!("Hello, world!");
    let j = thread::spawn(|| {
        println!("run in child thread");
        sleep(Duration::from_millis(100));
    });
    _ = j.join();
    println!("all finished");
}
