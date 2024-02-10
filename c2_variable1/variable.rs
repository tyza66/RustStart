fn main(){
    // rust是强类型语言，但是忽悠自动判断变量类型的能力
    // rust中有四种标量类型：整型、浮点型、布尔型、字符型
    let food = "清蒸螃蟹";  // string 字符串类型
    let price = 96.6;       // float 类型
    let checked = true;   // boolean 类型

    // 测试输出
    println!("food is:{}", food); 
    println!("price is:{}", price);
    println!("checked is :{}", checked);

    // 整形 u代表无符号 i带代表有符号 后面那个数字i代表中的是占用多少位
    // 数字位置放size的情况下就是说这个数字是根据系统位数来的
    let i0 = 20;
    let i1:u32 = 200;
    let i2:i32 = 200;
    let i3:isize = 200;
    let i4:usize = 200;

    // 测试输出
    println!("i0 is:{}", i0);
    println!("i1 is:{}", i1);
    println!("i2 is:{}", i2);
    println!("i3 is:{}", i3);
    println!("i4 is:{}", i4);

    // rust中声明的和赋值的类型是必须匹配的
    // 浮点型只有两种大小 f32和f64
    let f0 = 3.0;
    let f1:f32 = 3.0;
    let f2:f64 = 3.0;
    let f3 = 1_000_000.1; // 我们可以使用下划线来增加数字的可读性

    

    // 测试输出
    println!("f0 is:{}", f0);
    println!("f1 is:{}", f1);
    println!("f2 is:{}", f2);
    println!("f3 is:{}", f3);
}