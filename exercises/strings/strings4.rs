// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    // 打印字符串切片
    println!("{}", arg);
}
fn string(arg: String) {
    // 打印String类型的值
    println!("{}", arg);
}

fn main() {
    // 调用string_slice，因为传入的是字符串切片
    string_slice("blue");
    // 调用string，因为to_string()返回的是String类型
    string("red".to_string());
    // 调用string，因为String::from返回的是String类型
    string(String::from("hi"));
    // 调用string，因为to_owned()是&str的方法，它返回String类型
    string("rust is fun!".to_owned());
    // 调用string，因为into()是实现了Into<T> trait的类型的通用方法，  
    // 在这里它将&str转换为String，因为string函数期望String类型
    string("nice weather".into());
    // 调用string，因为format!宏返回String类型
    string(format!("Interpolation {}", "Station"));
    // 调用string_slice，因为&String::from("abc")[0..1]是一个字符串切片
    string_slice(&String::from("abc")[0..1]);
    // 调用string_slice，因为trim()返回的是字符串切片
    string_slice("  hello there ".trim());
    // 调用string，因为replace()返回的是新的String类型
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
