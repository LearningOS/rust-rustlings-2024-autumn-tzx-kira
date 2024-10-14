// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.



pub fn is_even(num: i32) -> bool {
    // 如果num除以2的余数为0，则num为偶数，返回true；否则返回false
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // 定义一个测试函数，用于测试当输入为偶数时，is_even函数是否返回true
    #[test]
    fn is_true_when_even() {
        assert!(is_even(4),"4 should be even");
    }

    // 定义一个测试函数，用于测试当输入为奇数时，is_even函数是否返回false
    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5), "5 should not be odd");
    }
}
