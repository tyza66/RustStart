fn main() {
    // 典型for循环
    for i in 1..5 { //默认这种是左闭右开区间
        println!("i = {}", i);
    }

    // 包含5
    for i in 1..=5 {
        println!("i = {}", i);
    }

    // 使用迭代器遍历
    let my_list = vec![
        "tyza66",
        "loong",
        "giao",
    ];
    for item in my_list.iter() { //这样遍历不会消耗集合 集合本身不会被改变
        println!("{}", item);
    }

    let my_list = vec![
        "tyza667",
        "loong",
        "giao",
    ];
    for item in my_list.into_iter() { //这样遍历会消耗集合 集合本身会被改变
        println!("{}", item);
    }

    let mut my_list = vec![
        "tyza668",
        "loong",
        "giao",
    ];
    for item in my_list.iter_mut() { //可变地遍历集合 这样遍历不会消耗集合 集合本身会被改变
        *item = "hello";
    }
    println!("{:?}", my_list);

    // while循环
    let mut i = 0;
    while i < 5 {
        println!("i = {}", i);
        i += 1;
    }

    // loop循环
    let mut j = 0;
    loop {
        println!("j = {}", j);
        j += 1;
        if j >= 5 {
            break;
        }
    }

}
