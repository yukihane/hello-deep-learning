// 3.3.2 行列の積
use ndarray::prelude::*;

/// 3.3.2 行列の積
fn main() {
    println!("3.3.2 行列の積");
    let a = array![[1.0, 2.0], [3.0, 4.0]];
    let b = array![[5.0, 6.0], [7.0, 8.0]];
    let result = a.dot(&b);
    println!("a: \n{}", a);
    println!("b: \n{}", b);
    println!("a・b: \n{}", result);

    println!();

    println!("3.3.2 行列の積(その2)");
    let a = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    let b = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    let result = a.dot(&b);
    println!("a: \n{}", a);
    println!("b: \n{}", b);
    println!("a・b: \n{}", result);

    // println!();

    // println!("3.3.2 行列の積(その3)");
    // let c = array![[1.0, 2.0], [3.0, 4.0]];
    // let result = a.dot(&c);
    // println!("a: \n{}", a);
    // println!("c: \n{}", c);
    // println!("a・c: \n{}", result);

    println!();

    println!("3.3.2 行列の積(3x2 ・ 2)");
    let a = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    let b = array![7.0, 8.0];
    let result = a.dot(&b);
    println!("a: \n{}", a);
    println!("b: \n{}", b);
    println!("a・b: \n{}", result);
}
