// 2.3 パーセプトロンの実装

use deep_learning_from_scratch::ch2::ch2_3_3::*;
use deep_learning_from_scratch::ch2::*;

fn main() {
    println!("2.3.1 簡単な実装");
    println!("AND(0,0): {}", ch2_3_1::and(0.0, 0.0));
    println!("AND(1,0): {}", ch2_3_1::and(1.0, 0.0));
    println!("AND(0,1): {}", ch2_3_1::and(0.0, 1.0));
    println!("AND(1,1): {}", ch2_3_1::and(1.0, 1.0));

    println!("2.3.3 重みとバイアスによる実装");
    println!("AND(0,0): {}", and(0.0, 0.0));
    println!("AND(1,0): {}", and(1.0, 0.0));
    println!("AND(0,1): {}", and(0.0, 1.0));
    println!("AND(1,1): {}", and(1.0, 1.0));

    println!("NAND(0,0): {}", nand(0.0, 0.0));
    println!("NAND(1,0): {}", nand(1.0, 0.0));
    println!("NAND(0,1): {}", nand(0.0, 1.0));
    println!("NAND(1,1): {}", nand(1.0, 1.0));

    println!("OR(0,0): {}", or(0.0, 0.0));
    println!("OR(1,0): {}", or(1.0, 0.0));
    println!("OR(0,1): {}", or(0.0, 1.0));
    println!("OR(1,1): {}", or(1.0, 1.0));
}
