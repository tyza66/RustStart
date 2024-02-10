fn main() {
    let mut v = Vec::new();
    v.push("Go语言极简一本通");
    v.push("Go语言微服务架构核心22讲");
    v.push("从0到Go语言微服务架构师");
    println!("len:{:?}",v.len());
    let s1=&v[1..3];
    println!("s1:{:?}",s1);
}
