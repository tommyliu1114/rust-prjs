fn main() {
    println!("Hellod, world!");
    let c = 'c';
let tup : (i32,f64,u8) = (500,6.4,1);
let (x,y,z) = tup;
println!("the value of y is : {} ",y);    
let y: (i32,f64,u8) = (500,6.4,1);
println!("y.0 is {},y.1 is {}, y.2 is {}",y.0,y.1,y.2);
//array 
let a = [1,2,3,4,5];
let b: [i32;5] = [4,5,6,7,8];
println!("b is {},{} ",b[1],a[0]);
}
