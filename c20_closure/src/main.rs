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


    let add = |x, y| x + y;
    receive_closure(add);

    let z = 3; //闭包函数仍然可以捕获到变量
    receive_closure1(|x| x + z);

    // 闭包作为参数和返回值
    let closure  = return_closure();
    println!("{}", closure(1));

    let closure1 = do1(add,1);
    println!("{}", closure1(2));

    let closure2 = do2(add,1);
    println!("{:?}", closure2(2));
}

// 函数接收一个闭包
fn receive_closure<F>(f: F)
    where F: Fn(i32, i32) -> i32
{
    let result = f(1, 2);
    println!("xxx{}", result);
}


fn receive_closure1<F>(f: F)
    where F: Fn(i32) -> i32
{
    let result = f(1);
    println!("yyy{}", result);
}

// 函数返回一个闭包
fn return_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// 参数和返回值都有闭包
fn do1<F>(f: F,x:i32) -> impl Fn(i32) -> i32
    where F: Fn(i32,i32) -> i32
{
    // 传进来的闭包f和参数x ，y需要调用返回的闭包的时候传入
    move |y| f(x,y)
}

// 通用形式的闭包方法
fn do2<F,X,Y,Z>(f:F,x:X) -> impl Fn(Y) -> Z
    where F: Fn(X,Y) -> Z,
    X: Copy
{
    move |y| f(x,y)
}