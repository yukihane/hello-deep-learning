use deep_learning_from_scratch::ch3::ch3_2_2::*;
use ndarray::prelude::*;

/// 3.2.2 ステップ関数の実装
fn main() {
    println!("3.2.2 ステップ関数の実装");
    let x = array![-1.0, 1.0, 2.0];
    let y = step_function(x.view());
    println!("input: {}", x);
    println!("output: {}", y);
}
