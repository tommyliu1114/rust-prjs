
/*
DSL 是 Rust 的宏中集成的微型 “语言”。
这种语言是完全合法的，
因为宏系统会把它转换 成普通的 Rust 语法树，
它只不过看起来像是另一种语言而已。
这就允许你为一些特定功能 创造一套简洁直观的语法（当然是有限制的）
*/
macro_rules! my_cal {
    (my_exec $e: expr) => {
        {
            let val: usize = $e;
            println!("{} = {}",stringify!($e),val);
        }
    };
    //下面开始可变参数的匹配
    (my_exec $e: expr,$(my_exec $es: expr),+) => {
        {
            my_cal!(my_exec $e);
            my_cal!($(my_exec $es),+);
        }
    }
}

fn main() {
    my_cal!(my_exec 2*3 + 12);
    my_cal!(my_exec (2*5)+10/3);
    println!("Hello, world!");
    //可变参数
    my_cal!(my_exec 23+3,my_exec 22*2,my_exec 11+2);
}
