
fn main(){
    let mut number = 1;
    while number != 4 {
        println!("{}",number);
        // 不支持自增，自减操作 ++ --
        // number++;
        number += 1;
    }
    // 看来是不支持for(i=0; i < 3; i++)
    let a = [1,2,3,4,5,5,6];
    // 迭代器
    for i in a.iter() {
        println!("value:{}",i)
    }   
    // 
    for i in 0..7 {
        println!("a[{}]:{}",i,a[i])
    }
    let s = ['H','e','l','l','o',' ','w','o','r','l','d','!'];
    let mut i = 0;
    // rust 原生的无限循环结构
    loop {
        let ch = s[i];
        if ch == ' '{
            break
        }
        println!("\'{}\'",ch);
        i += 1;
    }

    i = 0;
    // loop 用作查找工具
    let location = loop {
        let ch = s[i];
        if ch == ' ' {
            break i
        }
        i += 1;
    };

    println!("\' \' indexed at: {}",location);
}