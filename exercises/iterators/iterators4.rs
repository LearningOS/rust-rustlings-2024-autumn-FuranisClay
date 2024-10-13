// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// 

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    //迭代器库函数应用
    //1. (1..=num).product() Iterator trait 提供的 product 方法可以将迭代器中所有元素相乘并返回结果。这里，(1..=num) 是一个迭代器，.product() 会对范围内的所有数字进行累乘。
    (1..=num).fold(1, |acc, x| acc * x) // fold 高阶函数
    // fold 方法会从迭代器的第一个元素开始迭代，将初始值 1 作为 acc 的初始值。
    // 对于迭代器中的每一个元素 x：
    // acc 与 x 相乘并将结果赋值给 acc。
    // 继续这个过程，直到迭代器中的所有元素都被处理完。
    // 最终，fold 返回的结果是 acc 的最终值，即从 1 到 num 的乘积。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
