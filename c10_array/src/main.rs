fn main() {
    // 数组中的成员的类型必须相同
    // 也可以使用[类型; 大小]的方式来定义数组
    // 数组下标还是从0开始
    // 数组是在栈中分配的，数组可以自动被借用成为 切片(slice)
    let arr: [i32; 3] = [1, 2, 3];
    println!("{:?}", arr);

    // 数组的长度是固定的，不能动态改变

    let arr2=[4, 5, 6,7];
    println!("{:?}", arr2);
    let arr3:[&str;3]=["";3];
    println!("{:?}", arr3);

    // 获取数长度
    println!("arr2.len()={}", arr2.len());

    // 遍历数组
    for i in 0..arr2.len() {
        println!("arr2[{}]={}", i, arr2[i]);
    }
    for i in arr2 {
        println!("{}", i);
    }
    for i in arr2.iter() {
        println!("{}", i);
    }

    // 如果想让数组可变 可以使用mut关键字
    let mut arr4 = [1, 2, 3];
    arr4[0] = 4;
    println!("{:?}", arr4);

    
}
