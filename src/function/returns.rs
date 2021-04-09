// 返回 () 空元组 称为 unit， 返回类型可省略
pub fn test_returns() -> () {
    println!("{}", inc(11));

    let a1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("is 7 in arr ? {}", find(7, &a1));
    println!("is 10 in arr ? {}", find(10, &a1));

    let (p2, p3) = pow_2_3(5);
    println!("{}, {}", p2, p3);

    // diverging();
}

fn inc(num: i32) -> i32 {
    // return num + 1;
    num + 1 // 作为基于表达式的语言，最后一个表达式的值 默认为函数的返回值
}

fn find(n: i32, a: &[i32]) -> bool {
    for i in a {
        if *i == n {
            return true;
        }
    }
    return false;
}

// rust不支持多返回值， 但是可以利用 元组实现多个值的返回
fn pow_2_3(n: i32) -> (i32, i32) {
    (n * n, n * n * n)
}

// 发散函数[diverging function]
// rust的一个特性，发散函数并不返回, 使用 ! 作为返回类型的表示, 但并不表示任何类型
fn diverging() -> ! {
    panic!("This function will never return")
}
