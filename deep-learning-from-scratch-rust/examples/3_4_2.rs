use deep_learning_from_scratch::ch3::{ch3_2_4::sigmoid, ch3_4_2::identity_function};
use ndarray::prelude::*;

/// 3.4.2 各層における信号伝達の実装
fn main() {
    println!("3.4.2 各層における信号伝達の実装");
    let x = array![1.0, 0.5];
    let w1 = array![[0.1, 0.3, 0.5], [0.2, 0.4, 0.6]];
    let b1 = array![0.1, 0.2, 0.3];

    println!("w1 shape: {:?}", w1.shape());
    println!("x shape: {:?}", x.shape());
    println!("b1 shape: {:?}", b1.shape());

    let a1 = x.dot(&w1) + &b1;
    let z1 = sigmoid(a1.view());
    println!("a1: {}", a1);
    println!("z1: {}", z1);

    let w2 = array![[0.1, 0.4], [0.2, 0.5], [0.3, 0.6]];
    let b2 = array![0.1, 0.2];

    println!("z1 shape: {:?}", z1.shape());
    println!("w2 shape: {:?}", w2.shape());
    println!("b2 shape: {:?}", b2.shape());

    let a2 = z1.dot(&w2) + &b2;
    let z2 = sigmoid(a2.view());

    let w3 = array![[0.1, 0.3], [0.2, 0.4]];
    let b3 = array![0.1, 0.2];

    let a3 = z2.dot(&w3) + &b3;
    let y = identity_function(a3.view());

    println!("y: {}", y);
}
