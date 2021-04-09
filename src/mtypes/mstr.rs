pub fn test_str() {
    let x = "static";

    let x: &'static str = "Hello";

    // 转义字符

    let z = "foo
bar";
    let w = "foo\nbar";

    assert_eq!(w, z);

    // +r  避免转义
    let d1: &'static str = r"abc \n abc";
    // 等价于
    let d2 = "abc \\n abc";

    assert_eq!(d1, d2);
}

pub fn test_string() {
    let x: &'static str = "rubik";

    let mut s1 = x.to_string();

    print!("{}\n", s1);

    s1.push_str(" cube");

    print!("{}\n", s1);

    let s = "rubik".to_string();

    fn use_str(s: &str) {
        println!("str: {}", s);
    }

    // impl Deref<Target=str> for String
    // *s 可以从 String 得到 一个 str
    // 但是 rust 中 str 无法直接存在, 所以需要 & ;
    // ! use_str(&s); 也可以编译通过是因为 rust编译器会在 &后加足够多的* 尽可能取满足 Deref
    // ! 但这个特性在某些情况下可能会失效
    use_str(&*s);

    // 存储在Vec里的一些字节
    let miao = vec![229, 150, 181];
    let _miao_len = miao.len() as u32; // 类型转换
                                       // 我们知道这些字节是合法的UTF-8编码字符串，所以直接unwrap()
    let meow: String = String::from_utf8(miao).unwrap();
    println!("meow:{}", meow);
}

pub fn test_string_index() {
    let s1 = String::from("我爱雯雯猪");
    // s1[1];  无法这样访问字符串

    for i in s1.as_bytes() {
        print!("{} ", i);
    }
    println!("");

    for i in s1.chars() {
        print!("{} ", i);
    }
    println!();

    assert_eq!(String::from("雯").chars().next(), s1.chars().nth(2));
}
