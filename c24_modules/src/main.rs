use mylib::adder::add;

fn main() {
    // 功能和模块的作用就是用于将函数和结构体按功能分组
    // mod定义私有模块 如果想要模块外部访问需要pub

    // 私有的模块只能在当前模块中访问 私有模块中所有函数和结构体都是私有的
    // pub关键字可以将模块中的函数和结构体变成公有的 从而可以在模块外部访问 公有的里面可以包含私有的

    let a = add(1, 2);
    println!("a = {}", a);

}