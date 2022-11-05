use std::io;
use std::io::Read;

/// Rust没有Exception
/// 可恢复的错误通过Result<T,E>类来处理
/// 不可恢复的通过panic!宏来处理

fn main(){
    // recoverable();

    //错误的传递
    // let r = f(120000);
    // if let Ok(v) = r {
    //     println!("ok: f(-1) = {}",{})
    // }else{
    //     println!("Err")
    // }

    // 错误类型
    test_kind();
}
// no recoverable
fn test_panic(){
    panic!("no recoverable");
    println!("hello");
}
use std::fs::File;
fn recoverable(){
    let f = File::open("hello.txt");
    match f {
        Ok(file)=>{
            println!("File opened successfully");
        },
        Err(err) => {
            println!("Failed to open the file: {}",err)
        }
    }
}
// 简化版
fn recoverable_simplfy() {
    let f = File::open("hello.txt");
    if let Ok(file) = f{
        println!("File opened successfully");
    }else {
        println!("Failed to open the file")
    }
    
}
//如果想让可恢复的错误变成不可恢复的，则调用Result的unwrap或者expect
fn recoverable_to_unrecoverable() {
    let f = File::open("hello.txt").unwrap();
    // 区别是expect可以向panic发送信息
    let f = File::open("hello.txt").expect("Failed to open.");
    
}

// 错误传递
fn f(i:i32) -> Result<i32, bool> {
    if i> 0{ Ok(i)}
    else{ Err(false)}
}

fn g(i:i32) -> Result<i32,bool> {
    let t = f(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b)
    };
}
// ？符号的实际作用是将Result类非异常的值直接取出
fn g1(i:i32) -> Result<i32,bool> {
    // 也可以直接通过在Result后面加？将错误直接返回出去
    let t = f(i)?;
    Ok(t) 
}

// kind 获取Err类型
fn read_test_from_file(path:&str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

fn test_kind(){
    let str_file = read_test_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}",s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No Such File");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}