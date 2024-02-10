fn main() {
    // 布尔类型
    let checked:bool = true;
    println!("checked is :{}", checked);

    // 字符类型 是字符串的基本组成 UNT-8作为基础编码
    let c:char = 'R';
    println!("c is :{}", c);

    // 复合类型

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构操作
    let (x, y, z) = tup;
    println!("x is :{}", x);
    println!("y is :{}", y);
    println!("z is :{}", z);

    // 数组
    let arr = [1, 2, 3, 4, 5];
    println!("arr[0] is :{}", arr[0]);

}
