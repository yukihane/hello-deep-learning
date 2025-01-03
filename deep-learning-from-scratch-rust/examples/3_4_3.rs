// 3.4.3 実装のまとめ

use deep_learning_from_scratch::ch3::ch3_2_4::sigmoid;
use deep_learning_from_scratch::ch3::ch3_4_2::identity_function;
use ndarray::prelude::*;
use std::collections::HashMap;

struct Network {
    weight: HashMap<String, Array2<f64>>,
    bias: HashMap<String, Array1<f64>>,
}
impl Network {
    fn new() -> Network {
        let mut weight = HashMap::new();
        weight.insert("W1".to_string(), array![[0.1, 0.3, 0.5], [0.2, 0.4, 0.6]]);
        weight.insert("W2".to_string(), array![[0.1, 0.4], [0.2, 0.5], [0.3, 0.6]]);
        weight.insert("W3".to_string(), array![[0.1, 0.3], [0.2, 0.4]]);

        let mut bias = HashMap::new();
        bias.insert("b1".to_string(), array![0.1, 0.2, 0.3]);
        bias.insert("b2".to_string(), array![0.1, 0.2]);
        bias.insert("b3".to_string(), array![0.1, 0.2]);

        Network { weight, bias }
    }
}

fn foward(network: &Network, x: ArrayView1<f64>) -> Array1<f64> {
    let w1 = network.weight.get("W1").unwrap();
    let w2 = network.weight.get("W2").unwrap();
    let w3 = network.weight.get("W3").unwrap();

    let b1 = network.bias.get("b1").unwrap();
    let b2 = network.bias.get("b2").unwrap();
    let b3 = network.bias.get("b3").unwrap();

    let a1 = x.dot(w1) + b1;
    let z1 = sigmoid(a1.view());

    let a2 = z1.dot(w2) + b2;
    let z2 = sigmoid(a2.view());

    let a3 = z2.dot(w3) + b3;
    identity_function(a3.view())
}

/// 3.4.3 実装のまとめ
fn main() {
    println!("3.4.3 実装のまとめ");
    let network = Network::new();
    let x = array![1.0, 0.5];
    let y = foward(&network, x.view());
    println!("y: {}", y);
}
