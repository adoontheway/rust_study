// 用于调试中完整的输出结构体
#[derive(Debug)]

// 结尾不需要; 属性用,分隔
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}
// 单元结构体:只有象征意义，无需任何成员
struct UnitStruct;

/// 元组结构体:
// 用于定义那些经常使用又不是很复杂的简单数据
struct Color(u8,u8,u8);

struct Rect {
    width: u32,
    height: u32
}
// 结构体方法
impl Rect {
    // 结构体方法， &self 好比lua里面的self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 结构体关联函数
    fn create(width:u32, height:u32) -> Rect {
        Rect {
            width,
            height,
        }
    }
}

fn test_rect() {
    let re1 = Rect {
        width:30,
        height: 50
    };
    let re2 = Rect::create(30, 50);
    println!("{}, {}", re1.area(), re2.area())
}

fn main(){
    let domain = String::from("www.github.com");
    let n = String::from("adoontheway");
    let name = String::from("adoontheway");
    let a = Site{
        domain,// 可以这样 等同 domain:domain
        // n,// 不可以这样，要同名
        name,
        nation: String::from("China"),
        found:2022
    };
    // 此处正常打印
    println!("{:#?}",a);

    let b = Site {
        domain: String::from("github.io"),
        ..a//此处不允许有逗号，
    };
    // 此处打印已经被移动了
    // println!("{:#?}",a);

    struct Vec2(i32,i32,i32);
    let v1 = Vec2(12,12,12);
    println!("{} {} {}",a.domain,b.nation,v1.1);
    
    // println!("{:#?}",v1);
    test_rect()
}