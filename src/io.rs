
fn main(){
    
    env_vars();
    // terminal_inputs();
    // read_file();
    // read_file_in_bytes();
    // read_in_stream();
    // write_file();
    // write_file_1();
    append_test();

}   
//env variables
fn env_vars(){
    let args = std::env::args();
    println!("{:#?}", args);
    for arg in args {
        println!("{}",arg)
    }
}
use std::io::stdin;
fn terminal_inputs() {
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");
    
    println!("your input is {}", str_buf);
}
use std::fs;
fn read_file() {
    let text = fs::read_to_string("./src/condition.rs").unwrap();
    println!("{}",text);
}

fn read_file_in_bytes(){
    let text = fs::read("./src/condition.rs").unwrap();
    println!("{:?}",text);
}

use std::io::prelude::*;
fn read_in_stream(){
    // todo ou8?
    let mut buffer = [0u8; 5];
    // 这里忘记了unwrap居然报错，File没有read方法
    let mut file = fs::File::open("./src/condition.rs").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}",buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}",buffer);
}

fn write_file(){
    // 这个是一次性写入，
    // 文件不存在会直接创建文件，
    // 文件存在的话会覆盖原先的内容
    fs::write("./test.txt","hello from rust").unwrap();
}

fn write_file_1(){
    let mut file = fs::File::create("./test.txt").unwrap();
    file.write(b"hello from rust 1").unwrap();
}
use std::fs::OpenOptions;
fn append_test() {
    let f = append_file();
    match f {
        Ok(())=> println!("it is ok."),
        Err(err)=> println!("failed")
    }
}
// 返回Result空元组
// File没有append方法，需要通过OpenOptions来实现
fn append_file() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true).open("./test.txt")?;//问号直接返回错误
    file.write(b"Append content")?;
    Ok(())
}
//OpenOptions可以设置打开权限，append, read, write
fn open_option_test() -> std::io::Result<()>  {
    let mut file = OpenOptions::new()
        .read(true).write(true).open("./test.txt")?;
    
        file.write(b"cover");
        Ok(())
}