// 
pub struct ClassName {
    filed : i32,
}
// 封装
impl ClassName {
    pub fn new(value:i32) -> ClassName {
        ClassName {
            filed: value
        }
    }

    pub fn public_method(&self) {
        println!("from public method");
        self.private_method()
    }

    fn private_method(&self){
        println!("from private method")
    }
}

// 继承：Rust没有提供继承相关的东西

//多态 通过trait实现