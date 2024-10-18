// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // 使用while let语句来循环处理optional_integers.pop()返回的结果
        while let Some(integer_option) = optional_integers.pop(){
            if let Some(integer) = integer_option{
                assert_eq!(integer, cursor as i8); // cursor是i32类型，需要转换为i8以匹配integer的类型
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
