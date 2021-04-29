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
    let mut statc = Vec::new();
    statc.push(1);
    statc.push(2);
    statc.push(3);
    while let Some(top) = statc.pop(){
        println!(" is noe : {}",top);
    }
    let vi = vec!['a','b','c'];
    for (index,value) in vi.iter().enumerate(){
        println!("{} is at index {}",value,index);
    }
}
