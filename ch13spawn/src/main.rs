use std::thread;
use std::time::Duration;
fn main() {
    let gcHandle = thread::spawn(||{
        for i in 1..100{
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("hi number {} from main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Hello, world!");
    gcHandle.join().unwrap();
}
