
fn main() {
    // vector_test();
    // str_test();
    map_test()
}
use std::collections::HashMap;
fn map_test() {
    let mut map = HashMap::new();
    // 插入元素
    map.insert("color","red");
    map.insert("size","10 m^2");
    println!("{:#?}", map);
    // 获取元素
    println!("{}", map.get("color").unwrap());
    // 遍历map：遍历的元素是键值对元组
    for p in map.iter() {
        println!("{:?}",p);
    }

    // insert 会直接覆盖
    // 这样可以在确认不存在的时候插入
    map.entry("color").or_insert("red");

    // 在确认有某个键的情况下，如果要直接修改对应的值，以下方法更快
    if let Some(x) = map.get_mut(&"color") {
        *x = "blue"
    }
}

fn str_test() {
    let one = 1.to_string();// int to string
    let two = 2.0.to_string();// float to string
    let three = "slice".to_string();//str to string

    let mut s = String::from("hello");
    s.push_str(" world");
    s.push_str("!");
    // 字符串追加
    println!("{}",s);
    
    let mut s1 = String::from(" from rust");
    // 字符串用+号拼接，可以直接与str拼接
    let s3 = s + "-" +&s1;
    println!("{}", s3);
    // 也可以使用format宏拼接
    let mut s4 = String::from(" woo");
    let mut s5 = String::from(" hoo");
    let s = format!("{} {}",s4,s5);
    println!("{}", s);

    // 字符串长度
    // 中文字的长度是3，因为是utf8编码
    // 要取字符个数需要用到 s.chars.count()
    println!("{}", s.len());

    // 字符串遍历
    for c in s.chars() {
        println!("{}",c);
    }

    // 从字符串取单个字符
    // 返回的其实也是Result Some('o')
    // nth是在迭代器中取出某值的方法，所以不要在遍历中使用，因为utf8每个字符长度不一定相等
    let a = s.chars().nth(2);
    println!("{:?}",a);

    // 截取字符串:遇到utf8可能会报错
    let sub = &s[0..2];
    println!("{:?}",sub);
}

fn vector_test(){
    // vec 即 Vector，是一个存放多值的单数据结构，该结构将同类型的值线性存放在内存中
    let mut vector:Vec<i32> = vec![1,2,4,8];
    vector.push(12);
    // push 追加单个元素
    vector.push(32);
    println!("{:?}",vector);

    let mut v1:Vec<i32> = vec![64,128];
    // append用于追加，要同类型才能append，切需要强行指定，编译器不推断
    vector.append(&mut v1);
    println!("{:?}",vector);

    // 因为vec的长度无法从逻辑上推断，
    // get方法无法保证一定取到值，
    // 所以get方法的返回值是Option枚举类，有可能为空
    // 这是一种安全的取值方法，书写比较麻烦，
    // 如果自己能保证能取到值的情况下，可以使用数组取值语法
    println!("{}", match v1.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });
    println!("{:?}" ,v1);
    // 这里在append的时候已经把所有子元素都给vector了，所以去不到
    // println!("{}" ,v1[0]);
    println!("{}",vector[0]);


    // 遍历vector
    for i in &vector {
        println!("{}",i);
    }
    // 迭代器遍历
    for i in vector.iter() {
        println!("{}",i);
    }
    // 遍历并修改
    let mut v2 = vec![100,32,35];
    for i in &mut v2 {
        *i += 50;
    }
    println!("{:?}" ,v2);
}