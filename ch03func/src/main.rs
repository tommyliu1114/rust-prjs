fn main() {
    println!("Hello, world!");
    let y = {
        let cc = 3;
        cc + 1
    };
    println!("y is {} ,five is {} ,plus_one is {} ",y ,five(),plus_one(5));
}

fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}