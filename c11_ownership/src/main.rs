fn main() {
    // 相当于是rust中的内存管理机制
    // rust中的内存管理机制是基于所有权的
    // rust中堆中的引用只能被一个变量拥有，如果将在这个变量的值赋值给其他变量那么之前那个变量就会自动失效
    // 在rust中只有变量的值只有两种操作：move和copy
    // move: 当变量赋值给另一个变量时，原变量的值会被移动到新变量上
    // copy: 当变量赋值给另一个变量时，原变量的值会被复制到新变量上

    // 一般基本数据类型赋值给另一个变量时(因为存在栈上)，会发生copy操作
    let a = 1;
    let b = a;
    println!("a={}, b={}", a, b);

    // 一般引用数据类型赋值给另一个变量时(因为存在堆上)，会发生move操作
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1={}, s2={}", s1, s2); // 这里会报错，因为s1的值已经被move到s2上了
    println!("s2={}", s2);

    // 借用所有权
    // 如果想要继续使用s1的值，可以使用引用的方式临时借用一下所有权
    let mut s1 = String::from("hello");
    // &s1 表示s1的引用 并且mut还让它可变
    change(&mut s1);
    println!("s1={}", s1);
    let  s2 = &s1;
    println!("s1={}, s2={}", s1, s2);

    // 悬垂引用不允许出现
    // 在rust中，引用的生命周期是由编译器自动推断的，但是有时候编译器推断的生命周期并不是我们想要的
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //
    //     &s
    // }
    // 伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。
    // 但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现
}

fn change(s: &mut String) {
    s.push_str(" world");
}
