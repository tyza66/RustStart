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
    let myList = vec![
        "tyza66",
        "loong",
        "giao",
    ];
    for item in myList.iter() {
        println!("{}", item);
    }

}
