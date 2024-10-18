// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.



// Option 类型是 Rust 标准库中的一个枚举类型
// Option 枚举有两个变体：Some 和 None。Some 变体包含一个值，表示存在某个值；None 变体表示没有值。
// Option 类型的常用方法
// is_some()：判断 Option 是否包含值，返回一个布尔值。
// is_none()：判断 Option 是否不包含值，返回一个布尔值。
// unwrap()：获取 Option 中的值，如果 Option 是 Some，则返回值；如果 Option 是 None，则触发 panic。
// unwrap_or(default)：获取 Option 中的值，如果 Option 是 Some，则返回值；如果 Option 是 None，则返回指定的默认值。
// expect(msg)：获取 Option 中的值，如果 Option 是 Some，则返回值；如果 Option 是 None，则触发 panic，并显示指定的错误消息。

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    // 如果time_of_day大于23，返回None
    if time_of_day > 23{
        return None;
    }
    if time_of_day < 22 {  
        Some(5)  
    } else {  
        Some(0)  
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        

        // let icecreams = maybe_icecream(12);
        // assert_eq!(icecreams, 5);

        // 要获取Option中包含的值，你应该使用match语句或if let语句
        // match maybe_icecream(12) {  
        //     Some(icecreams) => assert_eq!(icecreams, 5), // 应该有5个冰淇淋  
        //     None => panic!("Expected Some but got None"), // 如果得到None则触发panic（这通常不会发生，除非函数实现有误）  
        // } 
        if let Some(icecreams) = maybe_icecream(12) {  
            assert_eq!(icecreams, 5); // 应该有5个冰淇淋  
        }else {  
            panic!("Expected Some but got None"); // 如果得到None则触发panic（这通常不会发生）  
        }

    }
}
