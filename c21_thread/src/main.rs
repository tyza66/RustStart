use std::thread;
use std::time::Duration;

fn main() {
    //子线程
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程 {}", i);
            // sleep 会阻塞当前线程 某个线程睡眠的时候会让出CPU
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程 当主线程结束时，子线程也会结束
    for i in 1..5 {
        println!("主线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等handler对应待子线程结束才继续 否则在这阻塞
    handler.join().unwrap();
}
