pub fn test() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0,
        radius: 1.0,
    };
    println!("circle c has an area of {}", c.area());
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// 泛型的 trait 约束
use std::fmt::Debug;
fn foo<T: Debug>(s: T) {
    println!("{:?}", s);
}

// 多trait 约束
fn foo1<T: Debug + Clone>(s: T) {
    s.clone();
    println!("{:?}", s);
}

// where关键字 从句
fn foo2<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn foo2_where<T, K>(x: T, y: K)
where
    T: Clone,
    K: Clone + Debug,
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// trait 默认方法
// is_invalid是默认方法，Foo的实现者并不要求实现它，如果选择实现它，会覆盖掉它的默认行为
trait Valid {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

// trait 继承
trait Foo {
    fn foo(&self);
}

trait FooBar: Foo {
    fn foobar(&self);
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) {
        println!("foo");
    }
}

impl FooBar for Baz {
    fn foobar(&self) {
        println!("foobar");
    }
}

// derive属性 [派生]
// Rust提供了一个属性derive来自动实现一些trait，这样可以避免重复繁琐地实现他们，
// 能被derive使用的trait包括：Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd
#[derive(Debug)]
struct DDD;

// impl Trait => after v1.26
// 既存型别(Existential Type)
trait Trait {
    fn method(&self);
}

impl Trait for i32 {
    fn method(&self) {}
}

// v1.26 之前的写法
// 使用Box ：即便回传的内容是固定的，但也会使用到动态内存分配
fn fooTraitBefore() -> Box<dyn Trait> {
    Box::new(5) as Box<dyn Trait>
}

// v1.26 开始
// 利用impl Trait 的写法可以避免使用Box
fn fooTraitAfter() -> impl Trait {
    5
}

// 其它受益用例
// 闭包
fn closureBefore() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn closureAfter() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// 传参
fn paramBefore<T: Trait>(x: T) {}

fn paramAfter(x: impl Trait) {}
