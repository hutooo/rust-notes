pub fn test() {
    // let mut buf = String::new();
    // read_from_stdin(&mut buf).unwrap();

    let data: &[u8] = &[1, 2, 3, 4];
    write_to_stdout(data).expect("write err");

    // let buf1 = "haha rubik".as_bytes();
    let buf1 = b"haha rubik";
    let filename = "/home/rubik/xspace/tmp/test0x01";
    create_file(filename, buf1).unwrap();

    let mut buf2 = String::new();
    read_file(filename, &mut buf2).unwrap();
    println!("file content: {}", buf2);

    opt_open_file(filename, b"hello even").unwrap();
}

use std::io::{self, Read};

macro_rules! mytry {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => return Err(::std::convert::From::from(err)),
        }
    };
}

fn read_from_stdin(buf: &mut String) -> io::Result<()> {
    match io::stdin().read_line(buf) {
        Ok(n) => {
            println!("ok {}", n);
        }
        Err(err) => println!("{}", err),
    };
    Ok(())
}

use std::io::Write;

fn write_to_stdout1(buf: &[u8]) -> io::Result<()> {
    mytry!(io::stdout().write(&buf));
    Ok(())
}

fn write_to_stdout(buf: &[u8]) -> io::Result<()> {
    // mytry!(io::stdout().write(&buf));
    // try! 宏 已经过时， 使用 ? 代替
    io::stdout().write(&buf)?;
    Ok(())
}

fn input_number() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // 这里等效的写法是：
    // let num: i32 = input.trim().parse().unwrap();
    let num = input.trim().parse::<i32>().unwrap();
    println!("您输入的数字是：{}", num);
}

macro_rules! numin {
    () => {{
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().unwrap()
    }};
}

fn input_number_macro() {
    let num: i32 = numin!();
    println!("您输入的数字是：{}", num);
}

use std::fs;

fn create_file(filename: &str, buf: &[u8]) -> io::Result<()> {
    let mut f = fs::File::create(filename)?;
    f.write(buf)?;
    Ok(())
}

fn read_file(filename: &str, buf: &mut String) -> io::Result<()> {
    let mut f = fs::File::open(filename)?;
    f.read_to_string(buf)?;
    Ok(())
}

use std::fs::OpenOptions;

fn opt_open_file(filename: &str, buf: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)?;
    file.write(buf)?;
    Ok(())
}
