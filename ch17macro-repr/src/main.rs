macro_rules! vec_strs {
    (
        // 重复开始：
        $(
            // 每次重复必须有一个表达式...
            $element:expr
        )
        // ...重复之间由“,”分隔...
        ,
        // ...总共重复0或多次.
        *
    ) => {
        // 为了能包含多条语句，
        // 我们将扩展部分包裹在花括号中...
        {
            let mut v = Vec::new();
            // 重复开始：
            $(
                // 每次重复将包含如下元素，其中
                // “$element”将被替换成其相应的展开...
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

 fn main() {
     let s = vec_strs![1, "a", true, 3.14159f32];
     assert_eq!(&*s, &["1d", "a", "true", "3.14159"]);
 }