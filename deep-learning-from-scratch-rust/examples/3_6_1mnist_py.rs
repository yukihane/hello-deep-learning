// mnist.py の移植コードを実行してみるテスト

use deep_learning_from_scratch::dataset::minst::download;

#[tokio::main]
async fn main() {
    let _ = download("train-images-idx3-ubyte.gz").await.unwrap();
}
