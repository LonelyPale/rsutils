// 非递归实现，效率高
pub fn non_recursive(n: i32) -> i32 {
    let mut result = 0;
    let mut first = 0;
    let mut second = 1;

    if n < 0 {
        panic!("input can not be less than zero!");
    } else if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    for _ in 2..=n {
        result = first + second;
        first = second;
        second = result;
    }

    return result;
}

// 递归实现，效率差
pub fn recursive(n: i32) -> i32 {
    if n < 0 {
        panic!("input can not be less than zero!");
    } else if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        recursive(n - 1) + recursive(n - 2)
    }
}

pub fn sequence(mut n: i32) {
    if n < 0 {
        panic!("input can not be less than zero!");
    }
    while n >= 0 {
        println!("fibonacci({})={}", n, non_recursive(n));
        n = n - 1;
    }
}

// rustfmt fibonacci.rs
// rustc fibonacci.rs
// rustfmt fibonacci.rs && rustc fibonacci.rs && ./fibonacci
