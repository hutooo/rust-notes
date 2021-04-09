pub fn test() {
    destruct1();
    destruct2();
    destruct3();
    destruct4();
    destruct5();
    destruct6();
    destruct7();
}

struct Point {
    x: i32,
    y: i32,
}

// 更加强大的解构
fn destruct1() {
    let p1 = Point { x: 1, y: 2 };
    match p1 {
        Point { x, y } => println!("{}, {}", x, y),
    }
    match p1 {
        Point { x: x1, y: y1 } => println!("{}, {}", x1, y1),
    }
    match p1 {
        Point { y, .. } => println!("{}", y),
    }
}

// 忽略及内存管理
fn destruct2() {
    let tuple: (u32, String) = (5, String::from("rubik"));

    let (x, s) = tuple; // String类型并未实现Copy, 所以tuple被整体move掉了
                        // 将导致编译错误
                        // println!("tuple is: {:?}", tuple);

    let tuple = (5, String::from("even")); // 变量遮蔽

    let (x, _) = tuple; // 忽略String类型，i32实现了Copy , tuple 不会被 move

    println!("tuple is {:?}", tuple);
}

// 范围和多重匹配
fn destruct3() {
    let x = 10;
    match x {
        // 0...10 => println!("0 ~ 10"),
        0..=10 => println!("0 ~ 10"), //  [...] 过时， 使用 [..=] 来表达闭区间
        _ => println!("other"),
    }

    let c = 'w';

    match c {
        'a'..='z' => println!("小写字母"),
        'A'..='Z' => println!("大写字母"),
        _ => println!("其它字符"),
    }
}

// 多重匹配
fn destruct4() {
    let x = 1;

    match x {
        1 | 2 => println!("1 or 2"),
        _ => println!("other"),
    }
}

//  ref  和  ref mut
fn destruct5() {
    let s1 = String::from("rubik");

    match s1 {
        s => println!("{}", s), // s 被 move
    }

    let s2 = String::from("even");

    match s2 {
        ref s => println!("{}", s),
    }
    // println!("{}", s1);  // s1 已经被 move， 无法使用
    println!("{}", s2);
}

#[derive(Debug)]
struct Person {
    name: Option<String>,
}

// 变量绑定
fn destruct6() {
    let x = 1;
    match x {
        e @ 1..=5 | e @ 10..=15 => println!("{}", e),
        _ => (),
    }

    let name = "Rubik".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });

    match x {
        Some(Person {
            name: ref a @ Some(_),
            ..
        }) => println!("{:?}", a),
        _ => {}
    }
}

// 后置条件
fn destruct7() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 if y => println!("yes"), // 伪代码： IF y AND (x IN List[4, 5])
        _ => println!("no"),
    }
}
