

/// todo String 为啥可以被修改，底层应该是有mut
/// 被切片引用的字符串是禁止修改的
/// 直接双引号的是str，rust的核心语言类型
/// String是Rust标准公共库的一种数据类型，功能更完善，支持字符串追加，清空等操作，比str多了一个capacity属性
fn main(){
    let s = String::from("0123456789");
    // 此处要用 &s 用s会在编译时报错
    // the size for values of type `str` cannot be known at compilation time
    let part1 = &s[0..5];
    let part2 = &s[3..8];
    println!("{} ---- {}",part1, part2);
    // x..y, 
    //..y 0-y 
    // y.. y-end
    // .. all

    // 字符串用尽量不要使用非英文字符，因为编码会有问题
    let s = String::from("哈哈哈hhhh");
    // 中文字符长度为3
    println!("{}",s.len());

    let s = "hello";
    let slice = &s[..2];
    println!("{}", slice);
    
    // 快速将String转为str多方法
    let s2 = s[..];

    // 数组也支持切片操作
}