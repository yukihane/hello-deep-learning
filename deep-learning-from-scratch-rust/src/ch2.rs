// 2章のコード
// 2.3 パーセプトロンの実装

/// 2.3.1 簡単な実装
pub mod ch2_3_1 {
    pub fn and(x1: f64, x2: f64) -> f64 {
        let w1 = 0.5;
        let w2 = 0.5;
        let theta = 0.7;
        let tmp = x1 * w1 + x2 * w2;
        if tmp <= theta {
            0.0
        } else {
            1.0
        }
    }
}

/// 2.3.3 重みとバイアスによる実装
pub mod ch2_3_3 {
    use ndarray::prelude::*;
    pub fn and(x1: f64, x2: f64) -> f64 {
        let x = array![x1, x2];
        let w = array![0.5, 0.5];
        let b = -0.7;
        let tmp = (w * x).sum() + b;
        if tmp <= 0.0 {
            0.0
        } else {
            1.0
        }
    }

    pub fn nand(x1: f64, x2: f64) -> f64 {
        let x = array![x1, x2];
        let w = array![-0.5, -0.5];
        let b = 0.7;
        let tmp = (w * x).sum() + b;
        if tmp <= 0.0 {
            0.0
        } else {
            1.0
        }
    }

    pub fn or(x1: f64, x2: f64) -> f64 {
        let x = array![x1, x2];
        let w = array![0.5, 0.5];
        let b = -0.2;
        let tmp = (w * x).sum() + b;
        if tmp <= 0.0 {
            0.0
        } else {
            1.0
        }
    }
}

/// 2.5.2 XOR ゲートの実装
pub mod ch2_5_2 {
    use super::ch2_3_3::*;

    pub fn xor(x1: f64, x2: f64) -> f64 {
        let s1 = nand(x1, x2);
        let s2 = or(x1, x2);
        and(s1, s2)
    }
}
