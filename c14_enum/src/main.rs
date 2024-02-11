// 枚举的定义
#[derive(Debug)]
enum IpAddrKind {
    Hello,
    World,
}

// option枚举
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}


fn main() {
    let four = IpAddrKind::Hello;
    let six = IpAddrKind::World;
    println!("four:{:?}", four);
    println!("six:{:?}", six);

    // 判断一个枚举的值 唯一的操作就是match
    let a = IpAddrKind::Hello;
    match a {
        IpAddrKind::Hello => println!("Hello"),
        IpAddrKind::World => println!("World"),
    }

    // 带数据类型的枚举
    #[derive(Debug)]
    enum IpAddr {
        V6(String),
    }
    let loopback = IpAddr::V6(String::from("::1"));
    match loopback {
        IpAddr::V6(s) => println!("s:{:?}", s),
    }
}

// option枚举通常用来作为返回值
// 如果有返回值。可以使用返回 Some(data)，如果函数没有返回值，可以返回 None
fn find_value(a: i32) -> std::option::Option<bool> {
    if a > 100 {
        Some(true)
    } else {
        None
    }
}

