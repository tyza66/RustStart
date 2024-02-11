fn main() {
    // 类型系统
    // rust不提供原生类型之间的隐式类型转换 但是可以使用as关键字进行隐式类型转换
    let a = 42;

    let b:f64 = a as f64;
    println!("b = {}", b);

    // 还可以在数字之后加上类型
    let c = 42u8;
    let d = 2u32;
    let e = 3f32;
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);

    // 如果啥也不写 默认整形使用i32 浮点数使用f64

    // 自动类型推断不光会看到变量的值 还会看到变量的使用方式
    let mut vec = Vec::new();
    vec.push(1);
    // vec.push(1.0); // 会报错 因为rust会根据第一个push的类型推断出vec的类型是i32

    // 别名 使用type能给已经存在的类型起一个别名 必须使用大驼峰命名法
    // 别名不是新类型 不能提供额外的安全性
    type NanoSecond = u64;
    type Inch = u64;

    let fff:NanoSecond = 5;
    let ggg:Inch = 5;
    println!("fff = {}", fff);
    println!("ggg = {}", ggg);

    // 类型转换
    // 一般使用trait解决类型转换的问题 一般会用到From和Into两个trait

    let s1 = "abc";
    let s2 = String::from(s1);
    println!("s2 = {}", s2);

    let s3 = MyNumber::from(30);
    println!("s3 = {:?}", s3);

    // 实现了from之后into是自带的
    let s4 = 5;
    let s5:MyNumber = s4.into();
    println!("s5 = {:?}", s5);

    // 将字符串转换成数字 只要是在from中实现了FromStr trait的类型都可以使用parse方法
    let s6:i32 = "5".parse().unwrap();
    println!("s6 = {}", s6);
}

#[derive(Debug)]
struct MyNumber {
    num: i32,
}

impl From<i32> for MyNumber {
    fn from(item: i32) -> Self {
        MyNumber { num: item }
    }
}
