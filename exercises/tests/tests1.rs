// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    // 使用 test 属性来标记这个函数是一个测试函数
    #[test]
    fn you_can_assert() {
        let x=1;
        // assert! 宏用于检查一个表达式是否为真（true）
        // assert!(条件); 如果条件为假，则触发失败
        assert!(x==1,"x should be 1");
    }
}
