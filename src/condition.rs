
fn main(){
    let number = 3;
    // 和golang是一样的
    // 条件处只接受bool值
    if number < 5 {
        println!("number < 5")
    }else if number < 10{
        println!("number < 10")
    }else{
        println!("number > 5")
    }

    // 三元运算
    let a = 3;
    let b = if a > 0 {1} else {-1};
    println!("b is {}",b)
}