/// 所有权
/// 每个值都有一个变量，称为其所有者
/// 一次只能有一个所有者
/// 当所有者不再程序运行的范围时，改值将被删除
/// 
/// Rust 之所以没有明示释放的步骤是因为在变量范围结束的时候，
/// Rust 编译器自动添加了调用释放资源函数的步骤
/// & 操作符将用于租借操作 borrow
/// 
/// 简明笔记：
/// 本质上就是在语言层面禁止了同一个可变数据会有多个变量引用的情况，
/// 一旦作为参数传递了，就会发生所有权的移动（Move）或借用（Borrow）。
/// 赋值给另一个变更也就自动放弃了所有权。从根本上杜绝了并发情景下的数据共享冲突。
fn main(){
    {
        // 大括号到本行之前不能使用
        let s = "hehe";
        // 本行到大括号之后可以使用
    }
    // 变量s在此无效
    // error:not in scope
    // println!("{}",s)

    /// 数据的交互方式有两种： 移动(move)与克隆(clone)
    let x = 5;
    // 将x的值复制给y
    let y = x;
    // 基本数据类型不需要存放到堆中，不会话费更长的时间和存储空间
    /// 基本数据类型包括:
    /// 所有数字类型
    /// bool
    /// float
    /// char
    /// 仅包含以上类型的tuple
    

    let s1 = String::from("hello");
    println!("{}, world",s1);
    // s2赋值时s1已经无效了：消化一下
    // let s2 = s1;
    // error: value borrowed here after move: s1
    // println!("{}, world",s1);

    // 克隆:s1和s2这样就是堆上的2个值
    let s2 = s1.clone();
    // s2声明有效
    println!("{} -- {}",s1,s2);

    take_owership(s1);
    // s1的值被放做参数出传入函数
    // 导致s1已经被移动，从这里开始失效
    // s1不可用

    let x = 3;
    // x 声明有效
    make_copy(x);
    // x的值被放做参数传入函数
    // 由于x是基本类型，所以依然有效
    // x可用

    let s3 = gives_owenership();
    // 获取返回值的所有权
    println!("s3:{}",s3);
    let s4 = String::from("Rust");
    // 声明有效
    let s5 = takes_and_gives_back(s4);
    // s4被当作参数移动，s5获得所有权
    // 函数结束时：
    // s3无效被释放
    // s4被移动
    // s5无效被释放

    // 引用与租借,引用不会获得所有权
    let s6 = String::from("heihei");
    // & 操作符用于取变量引用
    let s7 = &s6;
    println!("{} {}", s6, s7);
    println!("{} ", get_length(s7));
    // 错误,编译时报错：
    // `s7` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // s7.push_str(" hehehe");
    // println!("{}",s7);

    let mut s6 = String::from("heihei");
    // &mut 用于取变量的可变引用
    // 直接用&还是会报上面一样的错误
    let s7 = &mut s6;
    s7.push_str(" hehehe");
    println!("{}",s7);
    
    // 不可变引用可以多次重复引用
    let s6 = String::from("heihei");
    let s7 = &s6;
    let s8 = &s6;
    println!("{} {} {}",s6,s7,s8);
    // 可变引用不可以多次重复引用
    let mut s6 = String::from("heihei");
    let s7 = &mut s6;
    let s8 = &mut s6;
    // error： immutable borrow occurs here
    // println!("{} {} {}",s6,s7,s8);
    // 这种设计主要出于对并发状态下发生数据访问碰撞的考虑，在编译阶段就避免了这种事情的发生



    // 垂悬引用 Dangling References
    // 放在有指针概念的编程语言里它就指的是那种没有实际指向一个真正能访问的数据的指针
    // 不一定是空指针，还有可能是已经释放的资源
    // 它们就像失去悬挂物体的绳子，所以叫"垂悬引用"
    let refer = dangle();
    println!("{} ",refer)
}
// expected named lifetime parameter : &String
fn dangle() -> &String {
    let s = String::from("as");
    &s
}

fn get_length(s: &String) -> usize{
    s.len()
}

fn gives_owenership() -> String {
    let s = String::from("hello");
    //s 声明有效
    return s;
    // s被当作返回值移动出函数
}

fn takes_and_gives_back(s:String)->String{
    // s声明有效
    s
    // 被当作返回值移出函数
}

fn take_owership(s:String){
    println!("{}",s)
}

fn make_copy(i:i32){
    println!("{}",i)
}