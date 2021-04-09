pub fn test() {
    match file_double1("/home/rubik/xspace/tmp/testfile1") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err {}", e),
    }

    match file_double2("/home/rubik/xspace/tmp/testfile1") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err {}", e),
    }

    match file_double3("/home/rubik/xspace/tmp/testfile1") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err {}", e),
    }
}

use std::fs::File;
use std::io::Read;
use std::path::Path;

// 读取文件中的内容[内容只有一个数字], 解析为数字，再乘2
fn file_double1<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents
                .trim()
                .parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|x| x * 2)
}

fn file_double2<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(e.to_string());
    };
    let x = match contents.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };
    Ok(x * 2)
}

use std::error::Error;

macro_rules! mytry {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => return Err(::std::convert::From::from(err)),
        }
    };
}

fn file_double3<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn Error>> {
    let mut file = mytry!(File::open(file_path));
    let mut contents = String::new();
    mytry!(file.read_to_string(&mut contents));
    let n = mytry!(contents.trim().parse::<i32>());
    Ok(2 * n)
}

use std::io;
use std::num;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = mytry!(File::open(file_path).map_err(CliError::Io));
    let mut contents = String::new();
    mytry!(file.read_to_string(&mut contents).map_err(CliError::Io));
    let n: i32 = mytry!(contents.trim().parse().map_err(CliError::Parse));
    Ok(2 * n)
}
