use std::ops::Deref;

fn main() {
    // Rust 可以在 堆 上存储数据。Rust 语言中的某些类型，如 向量 Vector 和 字符串对象 String 默认就是把数据存储在 堆 上的
    // 为了避免内存泄漏和重复释放内存，Rust 语言提供了智能指针类型
    // Rust 语言把指针封装在智能指针对象中，智能指针对象实现了 Deref 和 Drop 两个特质中
    // Deref 用于重载解引用操作符 *，Drop 用于释放内存
    // Rust 语言提供了几种智能指针类型，如 Box、Rc、Arc、Cell、RefCell、Mutex、RwLock 等
    // Box 智能指针类型
    // Box 智能指针类型是 Rust 语言提供的最简单的智能指针类型
    // 当一个结构体实现了以上的接口后，它们就不再是普通的结构体了。
    // Rust 提供了在 堆 上存储数据的能力并把这个能力封装到了 Box 中。
    // 这种把 栈 上数据搬到 堆 上的能力，我们称之为 装箱。
    let a = 6;           // 默认保存在 栈 上
    let b = Box::new(a); // 使用 Box 后数据会存储在堆上
    println!("b = {}", b);// 输出 b = 6

    // 解引用
    let price1 = 158;           // 值类型数据
    let price2 = Box::new(price1); // price2 是一个智能指针，指向堆上存储的数据 158
    println!("{}", 158 == price1);
    // println!("{}",158==price2); 这样比较的情况下，price2 是一个智能指针，不能直接和 158 比较
    println!("{}", 158 == *price2); // 为了访问 price2 存储的具体数据，需要解引用

    // Deref 特质
    struct CustomBox<T> {
        value: T,
    }

    impl<T> CustomBox<T> {
        fn new(v: T) -> CustomBox<T> {
            CustomBox { value: v }
        }
    }

    impl<T> Deref for CustomBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.value
        }
    }

    let x = 666;
    let y = CustomBox::new(x);  // 调用静态方法 new() 返回创建一个结构体实例

    println!("666==x is {}", 666 == x);
    println!("666==*y is {}", 666 == *y);  // 解引用 y
    println!("x==*y is {}", x == *y);  // 解引用 y

    // Drop 特质 当超出了它的作用域范围时会触发调用 drop() 方法
    impl<T> Drop for CustomBox<T>{
        fn drop(&mut self){
            println!("drop CustomBox 对象!");
        }
    }
}
