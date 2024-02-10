fn main() {
    // if语句
    let total: f32 = 666.00;
    if total > 500.00 {
        println!("大于500")
    }

    // if else语句
    if total > 600.00 {
        println!("大于600")
    } else {
        println!("小于600")
    }

    // if else if语句
    if total > 700.00 {
        println!("大于700")
    } else if total > 600.00 {
        println!("大于600")
    } else {
        println!("小于600")
    }

    // match语句
    let total = 666;
    // 执行代码块
    match total {
        500 => println!("等于500"),
        600 => println!("等于600"),
        666 => println!("等于666"),
        _ => println!("不等于500、600、666"),
    }
    // 返回值
    let result = match total {
        500 => "等于500",
        600 => "等于600",
        666 => "等于666",
        _ => "不等于500、600、666",
    };
    println!("{}", result);
}
