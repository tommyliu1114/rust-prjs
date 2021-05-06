//宏的参数使用一个美元符号 $ 作为前缀，并使用一个指示符（designator）来 注明类型：
/*

    block
    expr 用于表达式
    ident 用于变量名或函数名
    item
    pat (模式 pattern)
    path
    stmt (语句 statement)
    tt (标记树 token tree)
    ty (类型 type)
 */
macro_rules! say_hi {
    () => {
        println!("hi!");
    };
}
macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name(){
            println!("you called {:?}()",stringify!($func_name))
        }
    };
}

create_function!(foo);
create_function!(bars);

macro_rules! print_res {
    ($expression: expr) => {
        println!("{:?} = {:?}",stringify!($expression),$expression)
    };
}

fn main() {
    say_hi!();
    foo();
    bars();
    print_res!(1u32 + 1);
    print_res!({
        let x = 1u32;
        x * x + 2 * x -1
    })
}

