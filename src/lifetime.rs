

fn main(){
    // bad_life();
    // str_test()
    str_slice();
}

fn bad_life(){
    // life a
    // let r;
    {// life b
        let x = 5;
        // error:borrowed value does not live long enough
        // r = &x;
    }
    // println!("r : {}",r)
}
// error:
// this function's return type contains a borrowed value, 
// but the signature does not say whether it is borrowed from `s1` or `s2`
// fn longer(s1:&str, s2:&str) -> &str {
//     if s2.len() > s1.len() {
//         s2
//     }else {
//         s1
//     }
// }

fn str_test(){
    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        // 这个报错
        // r = longer(s1,s2)
        // 这个加了生命周期注释语法就不报错
        r = longer_comment(s1,s2)
    }
    println!("{} is longer", r)
}
/// 生命周期注释是描述引用生命周期的办法
/// 虽然这样并不能改变引用的生命周期，但可以在合适的地方声明2个引用的生命周期一致
/// 生命周期注释语法：&'a type
/// eg：&'a i32, &'a str, &'a mut i32
/// 使用方式和泛型一样
fn longer_comment<'a>(s1:&'a str,s2:&'a str) -> &'a str{
    if s2.len() > s1.len() {
        s2
    }else {
        s1
    }
}

fn str_slice(){
    struct Str<'a> {
        content: &'a str
    }
    impl<'a> Str<'a>{
        fn get_content(&self) -> &str{
            self.content
        }
    }
    let s =Str {
        content: "string_slice"
    };
    println!("s.content = {}", s.content);
}

/// 静态生命周期
/// 生命周期注释有一个特别的：'static 。
/// 所有用双引号包括的字符串常量所代表的精确数据类型都是 &'static str ，
/// 'static 所表示的生命周期从程序运行开始到程序运行结束。
use std::fmt::Display;
// 泛型，特性与生命周期协同作战
fn longest_with_an_announcement<'a, T>(x:&'a str,y:&'a str, ann:T) -> &'a str where T: Display{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    }else{
        y
    }
}