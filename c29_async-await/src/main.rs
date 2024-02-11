use std::future::Future;
use async_std::task::{block_on, sleep, spawn};

#[async_std::main]
async fn main() {
    // // do3();
    // // do4();
    // let do3_span = spawn(do3);
    // let do4_span = spawn(do4);
    //
    // // 主线程必须等do3和do4执行完
    // do3_span.join().unwrap();
    // do4_span.join().unwrap();

    // let do5_spawn = spawn(do5());
    // // 直接调用do6() 会报错 因为do6()是一个异步函数 必须在async函数中调用 后面街上await
    // do6().await;
    // // 主线程必须等do5和do6执行完
    // do5_spawn.await;

    let result = block_on(test2());
    println!("{}", result);
}

// async 异步函数
async fn test() ->String{
    String::from("test")
}

fn test1() ->impl Future<Output = String>{
    async{
        let x = test().await;
        x + "9"
    }
}

fn test2() ->impl Future<Output = String>{
    let r = |x:String| async move{
        // y是test9
        let  y = test1().await;
        // 它是这个闭包函数的返回值
        y + &*x
    };
    // 传入x为6
    r(String::from("6"))
}

// fn do3(){
//     for i in 1..10 {
//         println!("{}", i);
//         sleep(Duration::from_millis(500));
//     }
// }
//
// fn do4(){
//     for i in 1..10 {
//         println!("{}", i);
//         sleep(Duration::from_millis(1000));
//     }
// }


// async fn do5(){
//     for i in 1..10 {
//         println!("do5 {}", i);
//         sleep(Duration::from_millis(500)).await;
//     }
// }
//
// async fn do6(){
//     for i in 1..10 {
//         println!("do6 {}", i);
//         sleep(Duration::from_millis(1000)).await;
//     }
// }
