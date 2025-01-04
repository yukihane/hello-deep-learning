// 3.5.3 ソフトマックス関数の特徴

use deep_learning_from_scratch::ch3::ch3_5_2::softmax;
use ndarray::prelude::*;

fn main() {
    println!("3.5.3 ソフトマックス関数の特徴");

    let a = array![0.3, 2.9, 4.0];
    let y = softmax(a.view());
    println!("y: {}", y);
    println!("sum(y): {}", y.sum());
}
