fn main() {
    // 泛型
    let mut v:Vec<i32> =vec![1,2,3];
    //v.push("4");//此处会报错 ^^^ expected `i32`, found `&str`

    // 泛型的定义
    struct Point<T> {
        x: T,
        y: T,
    }
    let p = Point { x: 5, y: 10 };
    let p1 = Point { x: 1.0, y: 4.0 };
    let p2: Point<i32> = Point { x: 5, y: 10 };

    // 泛型的方法
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // 特质
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    // 泛型函数
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// 特质
// 特质是一种定义签名的方法集合 相当于java的接口
// 特质的定义
pub trait Summary {
    fn summarize(&self) -> String;
}
// 特质的实现
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 泛型函数 <>里面的T是泛型（后面跟的是特质） h后面是参数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
