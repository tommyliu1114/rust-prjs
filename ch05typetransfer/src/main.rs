#![allow(overflowing_literals)]
#[allow(non_camel_case_types)]
type NanoSecond = u64;
type Inch = u64;
use std::mem;
use std::convert::From;
//From into 两个trait，用来解决类型转换

use std::convert::TryFrom;
use std::convert::TryInto;//用于试图转变类型

//转换为字符串，从字符串解析
use std::string::ToString;

struct Circle {
    radius: i32,
}

impl ToString for Circle{
    fn to_string(&self) -> String{
        format!("circle of radius {:?}",self.radius)
    }
}

#[derive(Debug,PartialEq)]
struct EvenNumber(i32);
impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self,Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        }else{
            Err(())
        }
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number{
    fn from(item: i32) -> Self {
        Number{value: item}
    }
}

fn main() {
    let num = Number::from(30);
    println!("my number is {:?} ",num);
    let derive_num: Number = 20.into();
    println!("my actual number is : {:?}",derive_num);//实现了from后，into自动获取
    println!("Hello, world!");
    assert_eq!(EvenNumber::try_from(8),Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5),Err(()));
    let c_circle = Circle{ radius: 6};
    println!("circle is : {}",c_circle.to_string()); //实现to_string方法即可输出字符串
    //实现了FromStr的trait即可实现parse方法
    let parsed: i32 = "5".parse().unwrap();
    println!("parsed num is {}",parsed);
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("128 as a i16 is : {}",128 as i16);

    //字面量
    let x = 23u8;
    println!("size of x in bytes: {}",std::mem::size_of_val(&x));
    //别名
    let ns: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;

    println!("{} nanoseconds + {} inches = {} uint?",ns,inches,ns+inches);
}


