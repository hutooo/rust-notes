pub fn test_func() {
    hi("rubik");
    say_what("rubik", hello);

    let xh = ("小何", 18);
    let lb = ("老布", 88);

    print_id(xh);
    print_id(lb);

    print_age(xh);
    print_age(lb);

    print_name(xh);
    print_name(lb);
}

fn hi(name: &str) {
    println!("Hi, {}", name);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn say_what(name: &str, func: fn(&str)) {
    return func(name);
}

fn print_id((name, age): (&str, i32)) {
    println!("name is {}, age is {}", name, age);
}

fn print_age((_, age): (&str, i32)) {
    println!("age is {}", age);
}

fn print_name((name, _): (&str, i32)) {
    println!("name is {}", name);
}
