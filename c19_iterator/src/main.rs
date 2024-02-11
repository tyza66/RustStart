fn main() {
    let v = vec!["apple", "giao", "banana", "mango", "orange"];
    let mut it = v.iter();
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next().unwrap());

    for i in it {
        println!("{:?}",i);
    }
}
