use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // 打开文件
    let file = std::fs::File::open("data.txt").unwrap();
    println!("文件打开成功\n:{:?}",file);
    // 创建文件
    let file = std::fs::File::create("data2.txt").expect("创建失败");
    println!("文件创建成功:{:?}",file);
    // 删除文件
    std::fs::remove_file("data2.txt").expect("删除失败");
    // 追加内容
    // 函数 append() 用于将文件的打开模式设置为 追加。
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("失败");
    // write可能只消耗部分数据，而不一定会将整个数据都写入 如果写入失败，它不会自动重试
    file.write("\ngiao".as_bytes()).expect("写入失败");
    println!("\n数据追加成功");

    // 写入所有内容
    // write_all方法确保传入缓冲区中的所有数据都被消耗
    // 如果 write 只消耗了部分数据，write_all 会自动重试，直到整个数据都被消耗（或发生无法恢复的错误）
    file.write_all("Rust".as_bytes()).expect("创建失败");
    file.write_all("Rust".as_bytes()).expect("创建失败");
    println!("数据已写入完毕");
}
