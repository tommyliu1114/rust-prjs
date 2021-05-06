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

//宏可以重载，从而接受不同的参数组合。在这方面，macro_rules! 的作用类似于 匹配（match）代码块

macro_rules! test {
    ($left: expr; and $right: expr) => {
        println!("{:?} and {:?} is {:?}",
    stringify!($left),
    stringify!($right),
    $left && $right)
    };

    ($left: expr; or $right: expr) => {
        println!("{:?} or {:?} is {:?}",
    stringify!($left),
    stringify!($right),
    $left || $right)
    };    
}

// `min!` 将求出任意数量的参数的最小值。
macro_rules! find_min {
    // 基本情形：
    ($x:expr) => ($x);
    // `$x` 后面跟着至少一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对 `$x` 后面的 `$y` 们调用 `find_min!` 
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    say_hi!();
    foo();
    bars();
    print_res!(1u32 + 1);
    print_res!({
        let x = 1u32;
        x * x + 2 * x -1
    });
    test!(1 + 1 == 2;and 2*21 == 4);
    test!(true; or false);


    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}

