

static Languae: &'static str = "rust";
//todo golbal variable , can change

const Threshold: i32 = 10;
//const golbal variable, can not change

fn is_big(n: i32) -> bool {
    n > Threshold
}

fn main() {
    let n = 16;
    println!("this is {}",Languae);
    println!("the threshold is {}",Threshold);
    println!("{} is {}",n, if is_big(n){"big"}else{"small"});
    println!("Hello, world!");
}
