// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



// 默认情况下，Rust 中的所有内容都是私有的，但有两个例外：pub trait 中的关联程序项默认为公有的；pub 枚举中的枚举变体也默认为公有的。当一个程序项被声明为 pub 时，它可以被认为是外部世界能以访问的。

mod sausage_factory {
    // 这个函数是私有的，外部模块无法访问它。 
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }
    // 这个函数被设计为私有（因为它没有被标记为 pub）
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    // 调用sausage_factory模块的make_sausage函数来制作香肠。
    sausage_factory::make_sausage();
}
