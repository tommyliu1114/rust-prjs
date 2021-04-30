fn main() {
    println!("Hello, world!");
    let cref = &4;
    match cref {
        &val => println!("got a value via destructing {:?}",val),
    }
    //值匹配
    match *cref {
        val => println!("got a value via derefencing {:?}",val),
    }
     // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
    let _not_a_reference = 3;

    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref _is_a_reference = 3;

    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字来创建引用。
    // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }   
    //match 解构结构体：
    struct Foo {x:(u32,u32),y:u32}
    let foo = Foo{x:(1,2),y:3};
    let Foo{x:(a,b),y} = foo;
    println!("a = {}, b= {}, y= {}",a,b,y);
    let Foo{y:i,x:j} = foo;
    println!("i={:?},j={:?}",i,j);

    //忽略某些变量
    let Foo {y,..} = foo;
    println!("y={}",y);
    //todo match guard 卫语句
    let pair=(2,-2);
    match pair{
        (x,y) if x==y => println!("x=y"),
        (x,y) if x + y == 0 => println!("positive+negative"),
        _ => println!("default")
    }

    match some_number() {
        Some(n @ 42) => println!("the answer is : {}",n),
        Some(n) => println!("not iteresting ... {}",n),
        _ => (),
    }
}


fn some_number() -> Option<u32> {
    Some(42)
}