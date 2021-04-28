use std::thread;
fn main() {
    let v = vec![1,2,3];
    let chandle = thread::spawn(move||{
        println!("here's a vector: {:?}",v);
    });
    drop(v);
    chandle.join().unwrap();
    println!("Hello, world!");
}
