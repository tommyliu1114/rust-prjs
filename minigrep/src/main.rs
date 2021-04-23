use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let (queryStr,fileName) = parse_config(&args);
    /* 
    let queryStr = &args[1];
    let fileName = &args[2];
    */
    let contents = fs::read_to_string(fileName).expect("something went wrong reading the file");
    println!("with text: \n{}",contents);    
    println!("Hello, world!");
}

fn parse_config(args: &[String]) -> (&str,&str){
    let query = &args[1];
    let fileName = &args[2];
    (query,fileName)
}