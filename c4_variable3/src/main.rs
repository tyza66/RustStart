fn main() {
    // rust中的变量默认是不可变的
    let hello = "Hello, world!";
    println!("{}", hello);
    // 重新赋值会报错
    //hello = "Hello, Rust!";
    // 必须要使用mut关键字来声明可变变量
    let mut hello = "Hello, world!";
    println!("{}", hello);
    hello = "Hello, Rust!";
    println!("{}", hello);

    //定义常量使用const关键字 常理定义必须指定类型
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // 如果对同一个名赋值这个叫重影 可以改变数据类型
    let name = "tyza66";
    let name = "hello";
    println!("name: {}", name);

    // 常量不能被隐藏也不能被重复定义
    //const MAX_POINTS: u32 = 100_000;
    //println!("MAX_POINTS: {}", MAX_POINTS);

    // static关键字声明全局变量
    static APPLE:&'static str = "果子";
    println!("APPLE: {}", APPLE);
    // 带有static声明的变量有固定的内存地址
    println!("APPLE address: {:p}", &APPLE);
    // 带有'static声明生命周期的，可以是可变的变量 但是需要使用static mut关键字
    // 除了str其他都要特别声明static的生命周期
}
