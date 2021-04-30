#![allow(unreachable_code)]

fn main() {
    
    println!("Hello, world!");
    let n = 230;
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number , increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, half the number");
        n / 2
    };
    println!("big_n is : {}",big_n);
    //loop 循环 break跳出循环，continue开始下一轮循环
    let mut count = 0u32;
    println!("let's count unitl infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}",count );
        if count == 5 {
            println!("ok ,that's enougth ! ");
            break;
        }
    }
    //break , continue  可以跳出或者继续循环嵌套标签
    'outer: loop {
        println!("entered the outer loop");
        'inner: loop {
            println!("entered the inner loop");
            break 'outer;
        }
        println!("this point will never be reached");
    }
    println!("exited the outer loop");


    //break 从loop循环中带出值
    let mut pcounter = 0;
    let presult = loop {
        pcounter += 1;
        if pcounter == 10 {
            break pcounter * 2 + 1;
        }
    };
    println!("finally presult is {}",presult);

    //while 循环
    let mut c_n = 1;
    while c_n < 101 {
        if c_n % 15 == 0{
            println!("fizzbuzz");
        } else if c_n % 3 == 0{
            println!("fizz");
        }else if c_n % 5 == 0 {
            println!("buzz");
        }else {
            println!("{}",c_n);
        }
        c_n += 1;        
    }
}
