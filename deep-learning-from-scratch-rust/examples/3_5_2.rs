// 3.5.2 ソフトマックス関数の実装上の注意

use ndarray::prelude::*;
use ndarray_stats::QuantileExt;

/// 3.5.2 ソフトマックス関数の実装上の注意
fn main() {
    println!("3.5.2 ソフトマックス関数の実装上の注意");

    let a = array![1010.0, 1000.0, 990.0];
    let result = a.map(|x: &f64| x.exp()) / a.map(|x: &f64| x.exp()).sum();
    println!("{}", result);

    let c = a.max().unwrap();
    println!("c: {}", c);
    let result = a.map(|x: &f64| (x - c));
    println!("a - c: {}", result);
    let result = result.map(|x: &f64| x.exp()) / result.map(|x: &f64| x.exp()).sum();
    println!("{}", result);
}
