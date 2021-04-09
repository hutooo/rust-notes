pub fn test_bool() {
    let x = false;
    let mut y = true;
    if y {
        println!("{}, {}", x, y);
    }
    y = false;
    println!("{}", y);
}

pub fn test_char() {
    let c = 'e';
    let cc = '宇';
    println!("{}, {}", c, cc);

    let x: &'static str = "m";
    let y = "慕";
    println!("{}, {}", x, y);
}

/// [T; N]  T:元素类型 N:数组长度  数组类型包含长度
pub fn test_array() {
    let a = [1, 2, 3, 4, 5];
    let b: [u8; 3] = [1, 2, 3];
    println!("a:{:?}, b:{:?}", a, b);

    fn inner(arr: [u8; 3]) {
        for i in &arr {
            println!("{}", i);
        }
    }

    inner(b);
    // inner(a); // 类型不匹配 [T;3] != [T;4]

    println!("{:?}", b); // 实现了 Copy Trait
}

pub fn test_slice() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice_complete = &arr[..]; // 获取 全部元素
    let slice_middle = &arr[4..6 + 1]; // 获取 中间元素
    let slice_right = &arr[6..9 + 1]; // 获取右侧元素
    let slice_left = &arr[0..4 + 1]; // 获取左侧元素

    println!(
        "{:?}, {:?}, {:?}, {:?}",
        slice_complete, slice_middle, slice_left, slice_right
    );

    fn show(arr: &[u8]) {
        for i in arr {
            print!("{}", i);
        }
        println!();
    }

    let a: [u8; 3] = [1, 2, 3];
    let slice_a = &a[..];
    show(slice_a);
    let b: [u8; 4] = [4, 5, 6, 7];
    show(&b[..]);
}

pub fn test_vec() {
    let mut v1 = vec![1, 2, 3]; // 通过 vec! 宏 来声明
    let v2 = vec![0; 10]; // 长度为0 全部元素为 0

    println!("{}", v2[0]);

    for i in &v1 {
        print!("{}", i);
    }
    println!();

    for i in &mut v1 {
        *i = *i + 1;
        print!("{}", i);
    }
    println!();
}

pub fn test_func() {
    fn foo(x: i32) -> i32 {
        x + 1
    }

    let f1: fn(i32) -> i32 = foo;

    let f2 = foo;

    assert_eq!(11, f1(10));
    assert_eq!(12, f2(11));
}
