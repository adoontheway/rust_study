/// 并发编程
/// 闭包：可以保存进变量或者作为参数传递给其他函数的匿名函数，
/// 闭包相当于Rust中的Lambda表达式
use std::thread;
use std::time::Duration;
// spawn函数是一个无参函数，
fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print:{}",i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main(){
    // thread::spawn(spawn_function);
    // 推荐使用匿名函数（闭包）
    // thread::spawn(|| {
    //     for i in 0..5 {
    //         println!("spawned thread print:{}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 0..3 {
    //     println!("main thread print {}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // 闭包
    // 闭包语法: |args..| ->return_type { body }
    let inc = |num:i32|->i32 {
        num +1
    };    
    // 闭包内的变量有他的作用域
    println!("inc(5) = {}",inc(5));//6
    println!("inc(2) = {}",inc(2));//3 

    // 闭包可以省略类型声明使用Rust自动类型推断机制
    let inc1= |num| {
        num +1
    };    
    // 闭包内的变量有他的作用域
    println!("inc(5) = {}",inc1(5));//6

    
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print1:{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print1:{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    // join 方法
    // 可以使子线程运行结束后再停止运行程序
    // 也就是主线程等待子线程执行完成
    handle.join().unwrap();

    let s = "hello";
    let handle1 = thread::spawn(|| {
        // 再子线程中尝试使用当前函数的资源，移动会报错，因为所有权的原因
        // println!("spawned thread print1:{}",s);
    });
    handle1.join().unwrap();

    // 可以使用move关键字来处理
    let handle2 = thread::spawn(move || {
        
        println!("s{}",s);
    });
    handle2.join().unwrap();

    msg_dispath();
}

use std::sync::mpsc;
// 消息传递
fn msg_dispath(){
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got : {}", received);
} 