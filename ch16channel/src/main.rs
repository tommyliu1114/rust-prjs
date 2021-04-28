use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx,rx) = mpsc::channel();
    let tHandle = thread::spawn(move || {
        let val = String::from("himeds");
        let cRe = tx.send(val);
    });
    tHandle.join().unwrap();
    let res = rx.recv();
    match res{
       Ok(ct) => println!("receive {} ",ct),
       _ => println!("error got",) 
    }
    println!("Hello, world!");
}
