#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32,f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}



fn main() {
    let name = "peter";
    let age = 27;
    let peter = Person{name,age};
    println!("{:?}",peter);

    let point: Point = Point{ x:0.3,y:0.4};
    println!("point coordinates: ({},{})",point.x,point.y);
    //todo local update structure
    let new_point = Point{x:0.1,..point};
    println!("second point : ({},{})",new_point.x,new_point.y);
    //todo destructure point
    let Point{x:my_x,y:my_y} = point;
    let _rectange = Rectangle{
        p1: Point{x:my_x,y:my_y},
        p2: point,
    };
    let _nil = Nil;
    let pair = Pair(1,0.1);
    println!("pair contains {:?} and {:?}",pair.0,pair.1);

    //todo destructure element tuples 
    let Pair(s_int,s_decimal) = pair;
    println!("pair contains {:?} and {:?}",s_int,s_decimal);
}
