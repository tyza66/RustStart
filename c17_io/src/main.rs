use std::io::Write;

fn main() {
    // 这个值最终会被传递给read_line方法，它会将用户输入的内容存入这个字符串中
    let mut in_word = String::new();
    // 返回的result是输入的字节数 unwrap()是一个帮助方法，用于简化可恢复错误的处理 read_line() 方法会自动删除行尾的换行符 \n
    let result = std::io::stdin().read_line(&mut in_word).unwrap();
    println!("您的输入是: {}", in_word);
    println!("读取的字节数为: {}", result);

    // 输出流
    let result1 = std::io::stdout().write("恐怖 ".as_bytes()).unwrap();
    println!("写入的字节数为: {}\n", result1);

    // 命令行参数
    let input_args = std::env::args();
    for arg in input_args{
        println!("命令行参数:[{}]",arg);
    }
}
