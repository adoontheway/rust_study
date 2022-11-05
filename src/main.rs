// 导入oop文件
// 使用其中的classname
mod oop;
use oop::ClassName;

fn main() {
    println!("Hello, world!");
    // 不声明mut是不可变变量
    // 变量声明之后类型就确定了，不能赋予不是同类型的值，损失精度的也不可以
    let a = 12;
    
    //  这里会报错，因为不能重新赋值
    // a = 13;
    // 以下2个不会报错
    // 但是会导致上面的a定义没用
    // 因为这里是重新绑定，也叫重影:Shadowing
    // 即：用同一个名字重新代表另一个变量尸体，其类型，可变属性和值都是可以变化
    // let a = 13;
    // let a:u64 = 123;

    // 声明mut是可变变量
    let mut b = 21;
    print!("a is {}", a);
    print!("a is {}, b is {}", a,b);
   
    b = 22;
    print!("a is {0}, a still is {0}, b is still{1}", a,b);
    // 常量定义
    // const c:i32 = 123;
    println!("this is the main module");

    let obj = ClassName::new(12);
    obj.public_method();
}