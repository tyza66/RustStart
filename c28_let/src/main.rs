fn main() {
    let s = Some("hello");
    let s1:Option<i32> = None;
    let s2:Option<i32> = None;

    // 如果ket将s结构成Some(i) 结构成功 则执行后面的代码
    if let Some(i) = s {
        println!("found a string: {}",i);
    }

    if let Some(i) = s {
        println!("found a string: {}",i);
    }else {
        println!("not found a string");
    }

    if let Some(i) = s1 {
        println!("found a string: {}",i);
    }else if true{
        println!("not found a string1");
    }else{
        println!("not found a string2");
    }

    let mut num = Some(0);
    while let Some(i) = num {
        if i > 9 {
            println!("greater than 9, quit");
            num = None; // 这样既可跳出循环
        }else{
            println!("i = {}", i);
            num = Some(i + 1);
        }
    }
}
