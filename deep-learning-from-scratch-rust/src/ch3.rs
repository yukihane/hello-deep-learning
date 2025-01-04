/// 3.2.2 ステップ関数の実装
pub mod ch3_2_2 {
    use ndarray::prelude::*;

    /// ステップ関数
    pub fn step_function(x: ArrayView1<f64>) -> Array1<f64> {
        x.map(|&x| if x > 0.0 { 1.0 } else { 0.0 })
    }

    pub fn step_function_for_graph(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

pub mod ch3_2_4 {
    use ndarray::prelude::*;
    use std::f64::consts::E;

    /// シグモイド関数
    pub fn sigmoid(x: ArrayView1<f64>) -> Array1<f64> {
        x.map(|&x| 1.0 / (1.0 + E.powf(-x)))
    }

    pub fn sigmoid_for_graph(x: f64) -> f64 {
        1.0 / (1.0 + E.powf(-x))
    }
}

pub mod ch3_2_7 {
    use ndarray::prelude::*;

    /// ReLU 関数
    pub fn relu(x: ArrayView1<f64>) -> Array1<f64> {
        x.map(|&x| if x > 0.0 { x } else { 0.0 })
    }

    pub fn relu_for_graph(x: f64) -> f64 {
        if x > 0.0 {
            x
        } else {
            0.0
        }
    }
}

/// 3.4.2 各層における信号伝達の実装
pub mod ch3_4_2 {
    use ndarray::prelude::*;

    pub fn identity_function(x: ArrayView1<f64>) -> Array1<f64> {
        x.to_owned()
    }
}

/// 3.5.1 恒等関数とソフトマックス関数
pub mod ch3_5_1 {
    use ndarray::prelude::*;

    /// ソフトマックス関数
    /// ch3_5_2 でオーバーフロー対策を行う
    pub fn softmax(x: ArrayView1<f64>) -> Array1<f64> {
        let exp_a = x.map(|&x| x.exp());
        let sum_exp_a = exp_a.sum();
        exp_a / sum_exp_a
    }
}

/// 3.5.2 ソフトマックス関数の実装上の注意
pub mod ch3_5_2 {
    use ndarray::prelude::*;
    use ndarray_stats::QuantileExt;

    /// ソフトマックス関数(オーバーフロー対策)
    pub fn softmax(x: ArrayView1<f64>) -> Array1<f64> {
        let c = x.max().unwrap();
        let exp_a = x.map(|&x| (x - c).exp());
        let sum_exp_a = exp_a.sum();
        exp_a / sum_exp_a
    }
}

#[cfg(test)]
mod tests {
    use ndarray::prelude::*;

    use super::ch3_2_2::*;

    #[test]
    fn it_works() {
        let input = array![-1.0, 1.0, 2.0];
        let _ = step_function(input.view());
    }
}
