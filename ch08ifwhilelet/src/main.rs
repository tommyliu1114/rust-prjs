fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供另一种失败情况下的条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };


    //if let 做match
    // 创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }  
    
    //while let
   // 将 `optional` 设为 `Option<i32>` 类型
   let mut optional = Some(0);

   // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
   // 执行语句块（`{}`）。否则就 `break`。
   while let Some(i) = optional {
       if i > 9 {
           println!("Greater than 9, quit!");
           optional = None;
       } else {
           println!("`i` is `{:?}`. Try again.", i);
           optional = Some(i + 1);
       }
       // ^ 使用的缩进更少，并且不用显式地处理失败情况。
   }
   // ^ `if let` 有可选的 `else`/`else if` 分句，
   // 而 `while let` 没有。     
}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}