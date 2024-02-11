pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 // 对标C++的命名空间 和java中的包
// 定义模块 并且在rust中允许多层的模块嵌套
pub mod adder {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}
