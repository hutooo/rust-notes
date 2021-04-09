pub fn test() {
    test1();
    test2();
    test3();
    test_rawptr1();
}

// 解引用 裸指针 *const T 和 * mut T
fn test1() {
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("raw points as {}", points_at);
}

// 读写一个 可变的 静态变量 static mut
fn test2() {
    static mut N: i32 = 5;
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}

// 调用一个不安全的函数
fn test3() {
    unsafe fn foo() {}
    unsafe {
        foo();
    }
}

// 不安全函数标示如果调用它可能会违反Rust的内存安全语意
unsafe fn danger_will_robinson() {
    // todo

    // unsafe block不安全块可以在其中调用不安全的代码
    unsafe {
        // todo
    }
}

// unsafe trait不安全trait及它们的实现，所有实现它们的具体类型有可能是不安全的
unsafe trait Scary {}
unsafe impl Scary for i32 {}

// 裸指针
// *const T
// *mut T

fn test_rawptr1() {
    // 创建裸指针
    let a = 1;
    let b = &a as *const i32;

    let mut a1 = 2;
    let b2 = &mut a1 as *mut i32;

    // 解引用需要在 unsafe 中进行
    println!("{}", unsafe { *b });
    println!("{}", unsafe { *b2 + 1 });

    // Box<T> 的 into_raw

    let x = Box::new(10);
    let x1 = &*x as *const i32;

    // into_raw
    let x2 = Box::into_raw(x);

    println!("{}", unsafe { *x1 });
    println!("{}", unsafe { *x2 + 9 });
}
