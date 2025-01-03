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
