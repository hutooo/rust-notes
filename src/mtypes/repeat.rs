pub fn test_for() {
    for i in 1..10 {
        println!("{}", i);
    }

    for (i, j) in (5..10 + 1).enumerate() {
        println!("i = {}, j = {}", i, j);
    }

    let lines = "Content of line one
    Content of line two
    Content of line three
    Contest of line for"
        .lines();

    for (num, line) in lines.enumerate() {
        println!("num: {}, line: {}", num, line);
    }
}

pub fn test_while() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

pub fn test_loop() {
    let a;
    loop {
        a = 1;
        break;
    }
    println!("{}", a);
}

pub fn test_break() {
    let mut x = 5;
    loop {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }
}

pub fn test_continue() {
    for x in 1..10 + 1 {
        if x % 2 != 0 {
            continue;
        }
        println!("x is :{}", x);
    }
}

pub fn test_label() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            }
            if y % 2 == 0 {
                continue 'inner;
            }
            println!("x:{}, y:{}", x, y);
        }
    }
}
