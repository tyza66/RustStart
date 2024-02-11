fn main() {
    // 闭包也叫做lambda 是一种能够捕获周围作用域中变量的函数

    // 无参数闭包
    // 闭包可以赋值给一个变量
    let f = || println!("Hello, world!");
    f();

    // 有参数闭包
    let add_one = |x| -> i32 { x + 1 };
    let add_two = |x| { x + 2 };
    println!("{}", add_one(1));
    println!("{:?}", add_two(1));

    // 闭包本质上很灵活，闭包可以在没有类型标注的情况下运行。可移动（move），又可借用（borrow）
    let x = 1;
    let add_x = move |y| x + y;
    println!("{:?}", add_x(1));
    println!("{}", x);


    // 主要是可以捕获周围作用域中的变量
    let x = 1;
    let add_x = |y| x + y; // x是一个周围作用域中的变量 y是传入的
    println!("{:?}", add_x(1));
}
