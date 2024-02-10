fn main() {
    // 定义结构体
    // 元组结构体
    struct Color(i32, i32, i32);
    // 经典结构体
    struct Point {
        x: i32,
        y: i32,
    }
    // 单元结构体
    #[derive(Debug)]
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
}
