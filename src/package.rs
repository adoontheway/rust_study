// 用来导入标准库
use std::f64::consts::PI;

/// package -> crate -> module(mod)
mod second_module;
mod nation {
    // 没有修饰符，所以是私有的
    pub mod city {
        fn govern(){
            println!("this is city govern")
        }
       
    }
    // 加了pub修饰符
   pub mod county{
       pub fn govern(){
            println!("this is county govern")
        }
    }
    // pub+use可以当作js的export来用
    // pub use county::govern;
    mod village {
        //内置对象也是默认私有，他的属性也是
        pub struct Haha {
            pub toast: String,
            fruit:String
        }
        pub fn govern(){
            // 可以通过super调用父级
            super::county::govern();
        }
    }
}
// use 关键字倒入模块
// 还可以 use crate::natiion::county as nation_county
use crate::nation::county;
fn main(){
    // error : private module city
    // crate::nation::city::govern();
    // crate::nation::county::govern();
    println!("this is the main module");
    println!("{}",second_module::message());

    county::govern();

    println!("{}", (PI/2.0).sin());
}

