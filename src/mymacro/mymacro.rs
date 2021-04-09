pub fn test() {
    test_macro_create_function();
    test_macro_repetition();
    test_macro_recursion();
    test_hygienic_macro();
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("function {:?} is called.", stringify!($func_name));
        }
    };
}

fn test_macro_create_function() {
    create_function!(foo);
    foo();
}

macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

fn test_macro_repetition() {
    let v1 = vector![1, 2, 4, 8, 10];
    println!("vector : {:?}", v1);
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn test_macro_recursion() {
    println!("min:{}", find_min!(1));
    println!("min:{}", find_min!(1 + 2, 2u32));
    println!("min:{}", find_min!(5, 2 * 3, 4));
}

// hygienic Macro 卫生宏

macro_rules! foo {
    () => {
        let x = 3;
    };
}

macro_rules! bar {
    ($v:ident) => {
        let $v = 3;
    };
}

macro_rules! item_foo {
    () => {
        fn item_foo_func() {
            println!("{}", 1);
        }
    };
}

fn test_hygienic_macro() {
    foo!();
    // println!("{:?}", x);
    bar!(a);
    println!("{}", a);
    item_foo!();
    item_foo_func();
}
