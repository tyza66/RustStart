fn main() {
    // 和其他语言的区别是 rust中没有自增自减运算符

    // +
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    // -
    let d = 10;
    let e = 20;
    let f = d - e;
    println!("{} - {} = {}", d, e, f);

    // *
    let g = 10;
    let h = 20;
    let i = g * h;
    println!("{} * {} = {}", g, h, i);

    // /
    let j = 20;
    let k = 10;
    let l = j / k;
    println!("{} / {} = {}", j, k, l);

    // %
    let m = 20;
    let n = 10;
    let o = m % n;
    println!("{} % {} = {}", m, n, o);

    // 逻辑运算符

    // 与运算
    let s = 20;
    let t = 10;
    let u = s & t;
    println!("{} & {} = {}", s, t, u);

    // 或运算
    let v = 20;
    let w = 10;
    let x = v | w;
    println!("{} | {} = {}", v, w, x);

    // 异或运算
    let y = 20;
    let z = 10;
    let aa = y ^ z;
    println!("{} ^ {} = {}", y, z, aa);

    // 左移运算
    let ab = 20;
    let ac = 10;
    let ad = ab << ac;
    println!("{} << {} = {}", ab, ac, ad);

    // 右移运算
    let ae = 20;
    let af = 10;
    let ag = ae >> af;
    println!("{} >> {} = {}", ae, af, ag);

    // 逻辑运算符
    let ah = true;
    let ai = false;
    let aj = ah && ai;
    println!("{} && {} = {}", ah, ai, aj);
    let ak = true;
    let al = false;
    let am = ak || al;
    println!("{} || {} = {}", ak, al, am);
    let an = true;
    let ao = !an;
    println!("!{} = {}", an, ao);
}
