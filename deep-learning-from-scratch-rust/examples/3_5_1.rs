// 3.5.1 恒等関数とソフトマックス関数

use ndarray::prelude::*;

/// 3.5.1 恒等関数とソフトマックス関数
fn main() {
    println!("3.5.1 恒等関数とソフトマックス関数");
    let a = array![0.3, 2.9, 4.0];
    let exp_a = a.map(|&a: &f64| a.exp());
    println!("exp_a: {}", exp_a);

    let sum_exp_a = exp_a.sum();
    println!("sum_exp_a: {}", sum_exp_a);

    let y = exp_a / sum_exp_a;
    println!("y: {}", y);
}
