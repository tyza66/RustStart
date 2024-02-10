fn main(){
    // rust是强类型语言，但是忽悠自动判断变量类型的能力
    // rust中有四种标量类型：整型、浮点型、布尔型、字符型
    let food = "清蒸螃蟹";  // string 字符串类型
    let price = 96.6;       // float 类型
    let checked = true;   // boolean 类型
    let y: f32 = 3.0; // f32

    println!("food is:{}", food); 
    println!("price is:{}", price);
    println!("checked is :{}", checked);
    println!("y is :{}", y);
}