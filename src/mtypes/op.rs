use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct A(i32);

impl Add for A {
    type Output = A;

    fn add(self, rhs: A) -> A {
        return A(self.0 + rhs.0);
    }
}

impl Sub for A {
    type Output = A;

    fn sub(self, rhs: A) -> A {
        return A(self.0 - rhs.0);
    }
}

pub fn test_overload() {
    let a1 = A(10i32);
    let a2 = A(5i32);
    let a3 = a1 + a2;
    println!("{}", a3.0);

    let a4 = a1 - a2;
    println!("{}", a4.0);
}

pub fn test_format() {
    let s = format!(
        "{1} 是一个有着{0:>0width$}KG重{height:?}cm高的大胖猪",
        75,
        "rubik",
        width = 4,
        height = 178
    );
    print!("{}\n", s);

    let n1 = 19;
    println!("{0:b}, {0:?}", n1);
}
