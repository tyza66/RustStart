fn main() {
    // 切片的大小是可以在运行过程中改变的
    let mut v = Vec::new();
    v.push("apple");
    v.push("banana");
    v.push("banana1");
    println!("len:{:?}",v.len());
    let s1=&v[1..3];
    println!("s1:{:?}",s1);
    first_word(s1);
    first_word1(&mut v);
    println!("v:{:?}",v[0]);
    // 除了字符串以外，线性数组也支持切片
}

// 切片可以用来当作参数
fn first_word(s:&[&str]){
    println!("s:{:?}",s[1]);
}

// 可变切片
fn first_word1(s:&mut[&str]){
    s[0]="apple96";
    println!("s:{:?}",s[0]);
}
