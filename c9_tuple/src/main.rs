fn main() {
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构操作
    let (x, y, z) = tup;
    println!("x is :{}", x);
    println!("y is :{}", y);
    println!("z is :{}", z);
    // 元组的访问
    println!("tup.0 is :{}", tup.0);
    println!("tup.1 is :{}", tup.1);
    println!("tup.2 is :{}", tup.2);

    // 元组的传参
    show_tuple((&tup.0.to_string(),&tup.1.to_string()));
}
fn show_tuple(tuple:(&str,&str)){
    println!("{:?}",tuple);
}