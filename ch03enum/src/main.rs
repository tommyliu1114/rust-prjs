#![allow(dead_code)]

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x:i64,y:i64}
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}",c),
        WebEvent::Paste(s) => println!("pasted {}",s),
        WebEvent::Click {x,y} => {
            println!("clicked at x = {}, y = {}.",x,y);
        },
    }
}

enum Status {
    Rich,
    Poor,
}

fn main() {
    println!("Hello, world!");
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text.".to_owned());
    let click = WebEvent::Click {x:20,y:90};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    //todo examples of use;
    use Status::{Poor,Rich};
    let c_status = Poor;
    match c_status{
        Poor => println!("the rich have lots of money!"),
        Rich => println!("the poor have no money!")
    }
    //todo discriminator enum
    println!("zero is {}",Number::One as i32);
    println!("roses are #{:06x}",Color::Red as i32);
}
//todo implicit discriminator , value from 0;
enum Number {
    Zero,
    One,
    Two,
}
//todo explicit discriminator 
enum Color {
    Red = 0xff000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}