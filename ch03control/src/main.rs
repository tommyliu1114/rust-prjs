fn main() {
    println!("Hello, world!");
    let numb = 4;
    if numb < 5 {
        println!("numb < 5 ")
    }else if numb > 10{
        println!("numb > 10")
    }else {
        println!("numb >= 5 && numb <= 10")
    }
    let res = if true {
        100
    }else {
        90
    };
    println!("res is {} ",res);
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
        //println!("againt");
    };
    println!("the result is {}.",result);
    let mut indenumber = 3;
    while indenumber != 0{
        println!("{}!",indenumber);
        indenumber = indenumber - 1;
    }
    println!("LIFTOFF!!!");
}
