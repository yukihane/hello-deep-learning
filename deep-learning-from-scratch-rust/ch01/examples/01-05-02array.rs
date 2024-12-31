// ref. https://docs.rs/ndarray/0.16.1/ndarray/doc/ndarray_for_numpy_users/index.html

use ch01::type_of;
use ndarray::prelude::*;

/// 1.5節の実行サンプル
fn main() {
    println!("1.5.2 NumPy配列の生成");
    let x = array![1., 2., 3.];
    println!("{}", &x);
    println!();

    println!("1.5.3 NumPyの算術計算");
    let y = array![2., 4., 6.];
    println!("x - y: {}", &x - &y);
    println!("x * y: {}", &x * &y);
    println!("x / y: {}", &x / &y);
    println!();

    println!("1.5.4 NumPyのN次元配列");
    let a: ArrayBase<ndarray::OwnedRepr<i32>, Dim<[usize; 2]>> = array![[1, 2], [3, 4]];
    println!("a: {}", &a);
    println!("a.shape(): {:?}", a.shape());
    println!("{:?}", type_of(&a));
    println!();

    let b = array![[3, 0], [0, 6]];
    println!("a + b: {}", &a + &b);
    println!("a * b: {}", &a * &b);
    println!("a * 10: {}", &a * 10);
    println!();

    println!("1.5.5 ブロードキャスト");
    let a = array![[1, 2], [3, 4]];
    let b = array![10, 20];
    println!("a * b: {}", &a * &b);
    println!();

    println!("1.5.6 要素へのアクセス");
    let x = array![[51, 55], [14, 19], [0, 4]];
    println!("x[0]: {}", &x.slice(s![0, ..]));
    // 別解 Convenience methods for 2-D arrays
    println!("x[0]: {}", &x.row(0));
    println!("x[0][1]: {}", &x.slice(s![0, 1]));
    println!("rows: ");
    for r in x.rows() {
        println!("{}", &r);
    }
    println!();
    let x = x.flatten();
    println!("flatten: {}", &x);

    println!("select: {}", &x.select(Axis(0), &[0, 2, 4]));

    let mask = x.mapv(|x| x > 15);
    println!("x > 15: {}", &mask);

    let y = x
        .iter()
        .zip(mask.iter())
        .filter_map(|(&val, &mask)| if mask { Some(val) } else { None });
    println!("filtered: {}", Array::from_iter(y));

    // 別解
    println!(
        "filtered: {}",
        Array::from_iter(x.into_iter().filter(|&x| x > 15))
    );
}
