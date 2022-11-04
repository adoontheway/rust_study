#[derive(Debug)]
// 可以这样
enum Book0 {
    Paperly,
    Electronic
}
// 这个玩意儿居然要对每个需要打印debug信息的struct加 而不是整个文件
#[derive(Debug)]
// 可以加类型
enum Book1 {
    Paperly(u32),
    Electronic(String)
}
#[derive(Debug)]
// 可以是结构体
enum Book2 {
    Paperly {index:u32},
    Electronic {url:String}
}

#[derive(Debug)]
// 可以是结构体
enum Book3 {
    Paperly (u32),
    Electronic {url:String}
}
/// rust  不支持switch
/// rust 通过match来实现分支结构
/// match 内也可以有返回值
fn main(){
    let book = Book1::Paperly(1001);
    let ebook = Book1::Electronic(String::from("https://www.ssss.com"));
    println!("{:?} vs {:?}", book, ebook);
    match_test();
    match_test1();
    match_test2();
    option_test();
}

fn match_test2() {
    let t = "abc";
    // match 不限制bool类型，还可以对整型，浮点型，字符和字符串类型进行分支选择
    match t {
        "abc" => println!("Yes"),
        _ => {}, // 这个是例外情况
    }
}

fn match_test1() {
    let book = Book3::Paperly(1001);
    match book {
        Book3::Paperly(i) => {
            println!("Paperly book {}", i)
        },
        // 没coverElectronic 居然也报错
        Book3::Electronic { url } => {
            println!("E-book {}", url)
        },
    }
}

fn match_test() {
    let book = Book2::Paperly { index : 1001 };
    let ebook = Book2::Electronic { url: String::from("google.com")};
    // match 可以有返回值，但是返回值类型都要一样
    // 如果枚举类型是元组，那么需要临时指定一个名字
    match book {
        Book2::Paperly { index } => {
            println!("Paperly book {}", index)
        },
        Book2::Electronic { url } => {
            println!("E-book {}", url)
        },
    }
}
/// Option是Rust标准库中的每句累，用于填补Rust不支持null的空白
enum Option<T> {
    Some(T),
    None,
}

fn option_test(){
    // let opt = Option::Some("hello");
    let opt:Option<&str> = Option::None;
    match opt {
        Option::Some(something)=>{
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}