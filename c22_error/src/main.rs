use std::fs::File;

fn main() {
    // 错误分为两类 一种是可恢复错误 一种是不可恢复错误
    // panic!() 不可恢复错误
    //panic!("出错啦"); // panic!() 会打印错误信息并退出程序
    println!("Hello, world!");

    // Result 枚举和可恢复错误
    // Result 枚举有两个成员 Ok 和 Err
    // Ok 成员表示操作成功
    // Err 成员表示操作失败
    // Result 枚举的定义

    //let f = File::open("abc.jpg"); //文件不存在，因此值为 Result.Err
    //println!("{:?}",f);

    // 返回值是unwrap是 Result<T, E>的方法，在实例上调用此方法时，如果是 Ok 枚举值，就会返回 Ok 中的对象，如果是 Err 枚举值，在运行时会 panic
    fn is_even(no:i32)->Result<bool,String> {
        return if no % 2 == 0 {
            Ok(true)
        } else {
            Err("输入值，不是偶数".to_string())
        }
    }

    let result = is_even(2).unwrap();
    println!("{:?}",result);

    //expect方法的作用和unwrap类似，区别在于，expect方法接受msg: &str作为参数，它在运行时的panic信息为format!("{}: {}", msg, error)，使用expect时，可以自定义报错信息，因此出现panic时比较容易定位
    let result = is_even(3).expect("错误情况");
    println!("相当于：{:?}",format!("错误情况: {}","giao"));


    // 可恢复的情况是，当我们知道可能出现错误的情况时，我们可以使用match表达式来处理错误
}
