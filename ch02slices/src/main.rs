use std::mem;
//array definition [T;size] 
//slice definition &[T]
fn analyze_slice(slice: &[i32]){
    println!("first element of the slice : {}",slice[0]);
    println!("the sliece has {} elements",slice.len());
}
fn main() {
    //array
    let xs: [i32;5] = [1,2,3,4,5];
    let ys: [i32;500] = [0;500];
    println!("first element of the array : {}",xs[0]);
    println!("second element of the array : {}",xs[1]);
    println!("array size: {}",xs.len());
    println!("array occupies {} bytes",mem::size_of_val(&xs));
    //array can be borrowed as slices
    analyze_slice(&xs);
    analyze_slice(&ys[1 .. 4]);
    println!("Hello, world!");
}
