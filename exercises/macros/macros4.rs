// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



// 宏定义中的分号（;）在宏规则的最后是可选的，但在某些情况下（特别是当宏体包含多个语句时）它们可能是必要的。

// #[rustfmt::skip]属性通常用于指示rustfmt工具跳过格式化特定代码块。
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
