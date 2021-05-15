// 我们定义一个简单的`Show`特性
// 然后分别为`u8`和`&str`实现`Show`特性
trait Show {
   fn show(&self);
}

impl Show for u8 {
    fn show(&self) {
        print!("{}", self);
    }
}

impl Show for &str {
    fn show(&self) {
        print!("{}", self);
    }
}
