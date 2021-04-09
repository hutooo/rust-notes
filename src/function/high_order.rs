//! 高阶函数
//! 它可以使用一个或多个函数作为参数，可以将函数作为返回值

pub fn test_high_order() {
    let func: IncType = inc;
    println!("{}", func(19));
    println!("{}", func(11)); // * rust中 函数的所有权无法转义，所以函数类型是引用类型.

    println!("{}", process(1, inc));
    println!("{}", process1(2, dec));

    let a = [1, 2, 3];
    let mut b = Vec::<i32>::new();
    for i in &a {
        b.push(get_func(*i)(*i));
    }
    println!("{:?}", b);
}

// 函数定义
fn inc(num: i32) -> i32 {
    num + 1
}

// 函数类型
type IncType = fn(i32) -> i32;

fn dec(num: i32) -> i32 {
    num - 1
}

fn process(n: i32, func: fn(i32) -> i32) -> i32 {
    func(n)
}

fn process1<F>(n: i32, func: F) -> i32
where
    F: Fn(i32) -> i32,
{
    func(n)
}

fn get_func(n: i32) -> fn(i32) -> i32 {
    if n % 2 == 0 {
        inc
    } else {
        dec
    }
}
