// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let x=1;
        let y=1;
        // assert_eq! 宏用于比较两个值是否相等，如果不相等，则会触发断言失败。
        assert_eq!(x,y,"x and y must be equal!");
    }
}
