// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.



mod delicious_snacks {
    // TODO: Fix these use statements
    // 使用句法 use p::q::r as x; 将编译目标名称重新绑定为新的本地名称。
    use self::fruits::PEAR as pear;
    use self::veggies::CUCUMBER as cucumber;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    pub const FRUIT: &'static str = pear;
    pub const VEGGIE: &'static str = cucumber;

}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::FRUIT,
        delicious_snacks::VEGGIE
    );
}
