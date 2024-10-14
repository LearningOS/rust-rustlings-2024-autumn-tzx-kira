// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



// 定义一个泛型函数，比较两个具有相同生命周期的字符串切片并返回较长的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 让两个string类型的作用域相同
    let string1 = String::from("long string is long");

    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    
    println!("The longest string is '{}'", result);
}
