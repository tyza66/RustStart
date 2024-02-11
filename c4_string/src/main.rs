fn main() {
    let name = "tyza66";

    let mut name0 = String::new();
    let name1 = String::from("tyza66");

    println!("{}", name);
    println!("{},{}", name0,name0.len());
    println!("{},{}", name1,name1.len());

    // 加入字符串
    name0.push('a');
    println!("{},{}", name0,name0.len());
    name0.push_str("tyza66");
    println!("{},{}", name0,name0.len());

    // 替换字符串
    let result = name1.replace("a", "b");
    println!("{},{}", result,result.len());

    let name2 = "hello".to_string();
    println!("{},{}", name2,name2.len());
    show(name2.as_str());

    // 去掉头尾的空白符
    let name3 = "  hello  ".to_string();
    println!("{},{}", name3,name3.len());
    println!("{},{}", name3.trim(),name3.trim().len());

    // 字符串分割
    let name4 = "hello world、rust";
    let name4s: Vec<&str> = name4.split("、").collect();
    for item in name4s {
        println!("{}", item);
    }

    // 字符串转换为字符
    let name5 = "hello";
    for item in name5.chars() {
        println!("字符：{}", item);
    }

    // 字符串拼接
    let name6 = "hello";
    let name7 = "world";
    let name8 = name6.to_string() + name7;
    println!("{}", name8);
}

fn show(name: &str) {
    println!("{}", name);
}
