fn main() {
    println!("Hello, world!");
    let y = {
        let cc = 3;
        cc + 1
    };
    println!("y is {} ,five is {} ",y ,five());
}

fn five() -> i32{
    5
}