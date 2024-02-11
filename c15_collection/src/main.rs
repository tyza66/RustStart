fn main() {
    // 标准库中提供了三种集合类型：Vec<T>、HashMap<K,V>、HashSet<T>
    // Vec<T> 是一个动态大小的数组
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v:{:?}", v);

    // 使用宏vec!来创建Vec<T>
    let v1 = vec![1, 2, 3];
    println!("v1:{:?}", v1);
    let contain = v1.contains(&1);
    println!("contain:{:?}", contain);

    use std::collections::HashMap;
    // new创建HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores:{:?}", scores);

    // 获取HashMap的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score:{:?}", score);

    // 遍历HashMap
    for (key, value) in &scores {
        println!("key:{},value:{}", key, value);
    }

    // 使用迭代器遍历
    for (key, value) in scores.iter() {
        println!("key:{},value:{}", key, value);
    }

    match scores.get(&*String::from("Blue")) {
        Some(v) => println!("v:{:?}", v),
        None => println!("None"),
    }

    // 判断是否包含key
    let contain = scores.contains_key(&String::from("Blue"));
    println!("contain:{:?}", contain);

    // 删除HashMap的值
    scores.remove(&String::from("Blue"));
    println!("scores:{:?}", scores);

    // 使用entry方法来插入值
    scores.entry(String::from("Blue")).or_insert(60);
    scores.entry(String::from("Yellow")).or_insert(60);
    println!("scores:{:?}", scores);

    // 创建HashSet
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("set:{:?}", set);

    // 使用宏创建HashSet
    let mut set1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    println!("set1:{:?}", set1);
    let contain = set1.contains(&1);
    println!("contain:{:?}", contain);
    let contain = set1.contains(&4);

    // 遍历HashSet
    for v in &set1 {
        println!("v:{:?}", v);
    }

    // 使用无序迭代器遍历
    for v in set1.iter() {
        println!("v:{:?}", v);
    }

    // 删除HashSet的值
    set1.remove(&1);
    println!("set1:{:?}", set1);


}
