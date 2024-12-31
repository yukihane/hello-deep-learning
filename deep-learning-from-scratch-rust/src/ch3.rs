/// 3.2.2 ステップ関数の実装
pub mod ch3_2_2 {
    use ndarray::prelude::*;

    /// ステップ関数
    pub fn step_function(x: ArrayView1<f64>) -> Array1<f64> {
        let iter = x.into_iter().map(|&x| if x > 0.0 { 1.0 } else { 0.0 });
        Array::from_iter(iter)
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
