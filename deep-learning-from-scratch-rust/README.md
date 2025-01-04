# ゼロから作るDeep Learning

本書通りPythonで進めようとしていたが、せっかくなのでRustで挑戦してみようと思う。

## NOTE

### 全般

- numpyの代わりに [ndarray](https://docs.rs/ndarray/latest/ndarray/) を利用
- matplotlib(pyplot)の代わりに [plotters](https://docs.rs/plotters/latest/plotters/) を利用

### max() 関数

3.5.2 で `max()` 関数を使うが、同等のものは [ndarray-stats クレート](https://docs.rs/ndarray-stats/latest/ndarray_stats/index.html) を導入すれば利用できる。

- [How do you get the maximum value from a rust ndarray?](https://stackoverflow.com/a/77157456/4506703)

