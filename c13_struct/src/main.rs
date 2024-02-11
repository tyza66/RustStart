// 经典结构体
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 定义结构体
    // 元组结构体
    struct Color(i32, i32, i32);
    // 单元结构体
    #[derive(Debug)] // 这个是用来方便调试的
    struct Unit;

    // 初始化结构体
    let mut black = Color(0, 0, 0);
    let origin = Point { x: 0, y: 0 };
    let unit = Unit;

    // 访问结构体的字段
    println!("black:{:?}", black.0);
    println!("origin:{:?}", origin.x);
    println!("unit:{:?}", unit);

    // 修改结构体内容
    black.0 = 255;

    // 结构体可以作为方法参数
    show_point(origin);

    // 结构体可以作为方法返回值
    let p = create_point(1, 2);
    println!("p:{:?}", p);

    // 方法定义使用关键字impl
    // 函数是不归诸任何实例对象的 但是方法是归属于实例对象的
    // &self表示当前结构体实例，也是结构体普通方法固定的第一个参数
    impl Point {
        fn show(&self) {
            println!("x:{},y:{}", self.x, self.y);
        }
    }

    let p1 = Point { x: 1, y: 2 };
    p1.show();

    // 结构体静态方法
    impl Point {
        fn create(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }

    let p2 = Point::create(3, 4);
    p2.show();

    // 单元结构体 是一个类型且只有一个值() 当一个函数不返回值的时候返回() 相当于void 单元结构体既是一个类型也是一个值

    // 元组结构体 里面可以有多个不同的数据类型 可以解构
    let color = Color(1, 2, 3);
    let Color(r, g, b) = color;
    println!("r:{},g:{},b:{}", r, g, b);
}

// 结构体可以作为方法参数
fn show_point(c: Point) {
    println!("p:{:?}", c);
}

// 结构体可以作为方法返回值
fn create_point(x: i32, y: i32) -> Point {
    Point { x, y }
}
