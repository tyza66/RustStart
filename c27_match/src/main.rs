fn main() {
    // 结构 &、ref、ref mut
    // 解引用 *

    let num = &100;

    match num {
        &val => println!("val = {}", val),
    }

    match *num {
        val => println!("val = {}", val),
    }


    // ref改变了赋值的行为 可以对具体的值创建引用
    let ref num3 = 100;
    match num3 {
        &val => println!("val = {}", val),
    }

    // 定义非引用变量 使用ref和ref mut仍然可以获得其引用
    let num31 = 100;
    match num31 {
        // val是i32引用 可以直接输出 也可以解引用输出
        ref val => println!("val = {}",*val),
    }

    let mut num4 = 100;
    match num4 {
        ref mut r => {
            *r += 10;
            println!("r = {}", r);
        }
    }
    let p = Point { x: 0, y: 7 };
    // 结构的时候省略了y
    let Point { x: a,.. } = p;
    println!("a = {}", a);
}

struct Point {
    x: i32,
    y: i32,
}
