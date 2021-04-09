// mod basetype {
//     include!("mtypes/basetype.rs");
// }

mod concurrent;
mod container;
mod error;
mod feature;
mod function;
mod lifetime;
mod mtypes;
mod mymacro;
mod pm;
mod primeval_point;
mod smartpoint;
mod stdio;

use concurrent::concurrent as coc;
use container::linear;
use error::opterr;
use feature::fet;
use function::{high_order, params, returns, stm_expr};
use lifetime::life;
use mtypes::{basetype, compound, mstr, op, repeat};
use mymacro::mymacro as mmc;
use pm::{matching, pattern};
use primeval_point::primeval;
use smartpoint::smartptr as sptr;
use stdio::stdio as stdiox;

use std::env;

fn main() {
    // 重复
    repeat::test_for();
    repeat::test_while();
    repeat::test_loop();
    repeat::test_break();
    repeat::test_continue();
    repeat::test_label();

    // 类型
    basetype::test_bool();
    basetype::test_char();
    basetype::test_array();
    basetype::test_slice();
    basetype::test_vec();
    basetype::test_func();

    // 复合类型
    compound::test_tuple();
    compound::test_struct();
    compound::test_enum();

    // 字符串
    mstr::test_str();
    mstr::test_string();
    mstr::test_string_index();

    // 运算符,重载
    op::test_overload();
    op::test_format();

    // 函数参数
    params::test_func();
    returns::test_returns();
    stm_expr::test_stm_expr();
    high_order::test_high_order();

    // 模式匹配
    matching::test();
    pattern::test();

    // trait
    fet::test();

    // 所有权 引用/借用 生命周期
    life::test();

    // 容器
    linear::test();

    // 错误处理
    opterr::test();

    // 输入输出
    stdiox::test();
    // 环境参数
    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }

    // macro 宏
    mmc::test();

    // 智能指针
    sptr::test();

    // 并发&并行
    coc::test();

    // unsafe && 原始指针
    primeval::test();
}

#[cfg(test)]
mod test {

    #[test]
    fn for_test() {
        for i in 1..10 {
            println!("{}", i);
        }
    }
}
