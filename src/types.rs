/**
 * 基础数据类型:
 * 数字
 * i8,u8
 * i16,u16
 * i32,u32
 * i64,u64
 * i128,u128
 * isize,usize:取决于目标平台架构即 32bit / 64bit
 * f32,f64 浮点型
 * 布尔
 * bool
 * 字符
 * char
 * 复合类型，
 * 元组 () 可以包含不同类型
 * 数组 [] 只能存放同类型数据
 */
/// 这个是文档注释
/// cargo doc 可以用来生成文档
fn main(){
    // 无符号8位整型0-255
    let a :u8= 0xff;
    println!("{}",a);
    // 有符号8位整形-127-127
    let a :i8 = 127;
    println!("{}",a);
    
    // 下划线用于分割数字，
    // 作用应该是类似于千分符在字符串中的作用：阅读的易读性
    // 打印的时候是会去掉下划线的
    let a :i32 = 97_123;
    println!("{}",a);

    // 浮点型赋值的时候需要小数点
    // 不然编译的时候会报错
    let a:f32 = 1.0;
    println!("{}",a);

    let a:f64 = 1.0;
    println!("{}",a);

    // 似乎都要给予初始值
    let b:bool = true;
    println!("{}",b);

    // 字符型只能用单引号,且长度只能是一个
    let c:char = 'a';
    println!("{}",c);

    // 复合类型
    let tup:(i32,char,u8) = (500,'c',12);
    // 不能直接{}打印，会报错
    // the trait `std::fmt::Display` is not implemented for `(i32, char, u8)`
    // 需要通过 {:?} 打印或者 {:#?} 进行美化
    println!("{:?}",tup);
    println!("{:#?}",tup);

    // 定义一个不可变数组
    let a = [1,2,3,4,5,6,6,7];//[1,2,3,4,5,6,6,7,]; 多一个逗号也是可以的
    println!("{:?}",a);
    // 会报错，因为是不可变数组，如果需要动态改变数组元素，需要给数组加mut修饰符
    // a[0] = 122;

    // 定义了类型与长度，如果值不是定义好的类型或者长度的话编译会不通过
    let a :[i32;5]=[1,2,3,4,5];
    println!("{:?}",a[1]);
    // 给a赋值5个100
    let a =[100;5];
    println!("{:?}",a);
}