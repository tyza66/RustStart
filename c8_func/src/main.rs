fn main() {
    hello();
    let name = get_name();
    println!("{}", name);
    let mut num = 1;
    double(num);
    println!("{}", num);
    double2(&mut num);
    println!("{}", num);
    let tup = (500, 6.4, 1);
    show(tup);
    let name = String::from("tyza66");
    show2(name);
    //对于复合类型，比如字符串，如果按照普通的方法传递给函数后，那么该变量将不可再访问
    //println!("{}", name);
}

fn hello() {
    println!("hello");
}

// 如果代码中美亚由return语句，那么代码块中的最后一行就是返回值
// 最后一行不必使用分号 因为分号表示语句结束 会返回一个空的元组
fn get_name() -> String {
    "tyza66".to_string()
}

// 这样是值传递 外部的num的值不会变
fn double(mut x: i32) {
    x = x * 2;
    println!("{}", x);
}

// 如果想引用传递并修改传入的那个外部的num的值
// *是解引用操作符 相当于&取引用的反向操作
fn double2(x: &mut i32) {
    *x = *x * 2;
    println!("{}", x);
}

// 复合类型传参
fn show(tup: (i32, f64, u8)) {
    println!("x is :{}", tup.0);
    println!("y is :{}", tup.1);
    println!("z is :{}", tup.2);
}

// 字符串传参
fn show2(name: String) {
    println!("{}", name);
}
