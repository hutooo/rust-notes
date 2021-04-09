pub fn test_tuple() {
    let e = (1, "even");
    let m = (0, "rubik");

    let (x, y) = e;

    println!("x:{}, y:{}", x, y);

    let i1 = m.0;
    let i2 = m.1;

    println!("i1:{}, i2:{}", i1, i2);
}

// 具名结构体
struct A {
    attr1: i32,
    attr2: String,
}

// 元组类结构体
struct B(i32, u16, bool);

// 空结构体 内存占用为0
struct D;

// 空结构体 rustc-v1.9 以后 也可以这么写
struct C {}

struct Person {
    name: String,
}

// 为 Person 实现成员函数
impl Person {
    // 类似 类函数
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    // 类似成员函数
    fn greeting(&self) {
        println!("{} say hello", self.name);
    }
}

// #[derive(Copy, Clone)]
struct T1 {
    id: i32,
}

impl T1 {
    fn new(id: i32) -> T1 {
        T1 { id }
    }

    fn show(&self) {
        println!("{}", self.id);
    }

    fn add_one(&mut self) {
        self.id += 1;
    }

    fn add_two(&mut self) {
        self.add_one();
        self.add_one();
        self.show();
    }
}

pub fn test_struct() {
    let mut t1 = T1::new(1);
    t1.show();
    t1.add_two();
}

// 枚举 [同C]
enum Direction {
    West,
    North,
    South,
    East,
}

// 枚举 [代数类型]
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
    // Vecc { x: i32, y: i32 },
}

pub fn test_enum() {
    let sp = SpecialPoint::Point(0, 0);
    match sp {
        SpecialPoint::Point(x, y) => {
            println!("sp({}, {})", x, y);
        }
        SpecialPoint::Special(s) => {
            println!("sp({})", s);
        }
    }
}
