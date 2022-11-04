/// 函数的定义: fn <fun_name>(<args>) -> <return_type> <body>

fn main(){
    hehe();
    let b:i32 = add(123,124);
    println!("{}",b);

    // 表达式块，如计算公式，判断等等写在{}里面
    let c = {
        let b = 3;
        b + 1//返回值
    };

    println!("{} {}",b,c);

    // 函数可以嵌套
    fn five()->i32{
        return 5
    }
    println!("{} ",five());
}

fn hehe(){
    // println不是函数，需要加上!来调用宏macro
    println!("hehe");
}
/// Rust不支持自动返回值类型判断，如果没有返回值类型，函数会被认为是“纯过程”
fn add(a:i32,b :i32) -> i32{
    return a+b
}