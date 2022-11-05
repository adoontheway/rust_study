

/// 泛型机制是编程语言用于表达类型抽象的机制，
/// 一般用于功能确定，数据类型待定的类，如链表，映射表等

fn main(){
    // 这是个普通的取最大值的程序，只能对i32类型使用
    let a = [2,4,3,54,65,23,2,14,65,667,32,1,2];
    println!("max = {}", max(&a));
    let b = [2.0,4.0,3.0,54.0,65.0,23.0,2.0,14.0,65.0,667.0,32.0,1.0,2.1];
    println!("max_g = {}", max_g(&b));

    // generic_test();

    trait_test();
}

fn max(array:&[i32]) ->i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

trait Comparable {
    fn compare(&self, object:&Self) -> i8;
}

impl Comparable for f64 {
    fn compare(&self, object:&Self) -> i8 {
        if &self > &object{1}
        else if &self == &object { 0 }
        else { -1 }
    }
}

fn max_g<T:Comparable>(array:&[T]) ->&T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0{
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

fn generic_test() {
    struct Point<T> {
        x:T,
        y:T
    }
    // 方法也应该实现泛型的机制，否则泛型的类将无法被有效的方法操作
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    // 不阻止内部方法有泛型能力
    // impl<T,U> Point<T,U> {
    //     fn mixup<V,W>(self, other:Point<V,W>) -> Point<T,W> {
    //         Point {
    //             x: self.x,
    //             y: other.y
    //         }
    //     }
    // }
    // 不能同时定义：方法重复了
    // 我们也可以为其中的一种泛型添加方法
    // impl Point<f64> {
    //     fn x(&self) -> f64 {
    //         &self.x
    //     }
    // }
    let p1 = Point { x:1,y:2};
    let p2 = Point { x:1.0, y :2.0};
    println!("p.x : {}", p1.x());
    println!("p.x : {}", p2.x());
    // 这个编译的时候会报错，因为需要是同一个类型 T
    // let p3 = Point {x:1, y:2.0};

    struct Point1<T1,T2> {
        x:T1,
        y:T2
    }
    // 两个不同的泛型符，允许同样的类型
    let p1 = Point1 { x:1,y:2};
    let p2 = Point1{ x:1.0, y :2.0};
    let p3 = Point1 {x:1, y:2.0};
}

/// trait 特性，与interface接近
/// 他是一种行为规范，可以用于标示哪些类有哪些方法
trait Descriptive {
    fn describe(&self) -> String;
    // 与接口不同点，接口只能规范方法不能定义方法，
    // 特性可以定义方法作为默认方法，
    // 所以对象可以重新定义方法
    // 也可以不重新定义方法
    fn describe1(&self) -> String {
        String::from("[Object]")
    }
}

struct Person {
    name : String,
    age: u8
}
/// 实现格式 impl trait_name for type_name
impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("Name:{}, Age:{}", self.name, self.age)
    }
}
/// 可以通过传递特性作为参数，其实也就是作为接口类型，应用场景：回调函数，按钮事件等
/// 涉及到多个特性的时候，可以用+号进行连接，如：impl Descriptive + Talkable + Showable
fn output(object:impl Descriptive){
    println!("{}", object.describe());
}
/// 也可以用类似泛型的语法糖
fn output_two<T: Descriptive>(arg1:T,arg2:T){
    println!("{}",arg1.describe());
    println!("{}", arg2.describe1());
}

/// 复杂的实现关系可以使用where关键字简化
// fn hehe<T,U>(t:T, u:U) -> i32 
//     where T: Display + Clone, U: Clone + Debug {

//     }

// trait作为返回值: 其实也就是接口的应用
// 注意： 特性做返回值只接受实现了该特性的对象做返回值且在同一个函数中所有可能的返回值类型必须完全一样
fn person() -> impl Descriptive {
    // 返回值类型必须完全一样
    Person {
        name: String::from("ali"),
        age: 15
    }
}

// 这段代码声明了A<T>类型必须在T已经实现B和C的特性的前提下才能有效实现此impl块
// struct A<T> {}

// impl<T:B + C> A<T> {
//     fn d(&self){}
// }

fn trait_test() {
    let  p = Person {
        name: String::from("ali"),
        age: 15
    };

    let  p1 = Person {
        name: String::from("abe"),
        age: 15
    };
    // 重写的方法
    println!("{}",p.describe());
    // 默认的方法
    println!("{}",p.describe1());
    output(p);
    // 以上p被move了
    let  p = Person {
        name: String::from("ali"),
        age: 15
    };

    let  p1 = Person {
        name: String::from("abe"),
        age: 15
    };
    output_two(p,p1);
}