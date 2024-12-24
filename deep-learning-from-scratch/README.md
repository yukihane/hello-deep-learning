ゼロから作る Deep Learning ―Python で学ぶディープラーニングの理論と実装

- https://www.oreilly.co.jp/books/9784873117584/

の写経。

公式リポジトリー

- https://github.com/oreilly-japan/deep-learning-from-scratch
  - 正誤表: https://github.com/oreilly-japan/deep-learning-from-scratch/wiki/errata

## Note

### Ch01

1.6 で Matplotlib を利用しているが、Ubuntu 22.04 で `matplotlib` でグラフ表示を行うのに `PyQt6` のインストールが必要だった。

- https://stackoverflow.com/a/78344937

また、 `PyQt6` をビルドするのに `clang` が必要だった(※WSL2のUbuntu22.04.5では何故か不要だった):

```bash
sudo apt install clang
```

### Ch03

3.2.2 で次のエラー:

```
AttributeError: module 'numpy' has no attribute 'int'.
```

> `np.int` は非推奨となり、numpy の新しいバージョンでは削除されました。代わりに、 `int` を直接使用するか、 `np.int32` または `np.int64` を使用してください。

とのこと。
[公式リポジトリー](https://github.com/oreilly-japan/deep-learning-from-scratch/blob/master/ch03/step_function.py)では `int` になっている。

次のリンク先に詳細:

- https://qiita.com/yusuke_s_yusuke/items/bf7ce2deb6153ab0123b
