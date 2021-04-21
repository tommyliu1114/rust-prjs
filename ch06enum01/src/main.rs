
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrStr {
    V4(String),
    V6(String),
}

enum IpAddrArrs {
    V4(u8,u8,u8,u8),
    V6(String),
}
fn main() {
    println!("Hello, world!");
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let ihome = IpAddrStr::V4(String::from("127.0.0.1"));
    let iloopback = IpAddrStr::V6(String::from("::1"));
    
}

fn route(ip_type: IpAddrKind){

}

enum message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
