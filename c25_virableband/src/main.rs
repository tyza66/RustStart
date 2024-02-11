fn main() {
    // 变量绑定
    // 变量是有作用域的 在一个代码段中生存 代码块也允许变量遮蔽（重影）

    // 也就是内部的局部变量会覆盖外部的局部变量 但是不会影响到外部的全局变量

    let a = 1;
    {
        let b = 2;
        println!("a = {}, b = {}", a, b);
        let a = 3;
        println!("a = {}", a);
    }
    // println!("a = {}, b = {}", a, b); // b is not found
    println!("a = {}", a);

    // 冻结 资源存在使用的引用时，在当前作用域中这一资源是不可被修改的
    let mut aaa = Box::new(1);
    // bbb是aaa的引用
    let bbb = &aaa;
    // aaa被冻结了 这时候不兴改变aaa的值
    aaa = Box::new(2);
    // bbb使用了
    println!("bbb = {}", bbb);
}
