# python

[pyautd](https://github.com/shinolab/autd3/tree/master/python)はpythonに対応したラッパーを提供している.

## Installation

[PyPI](https://pypi.org/project/pyautd3/)で公開しているので, pipからインストールすること.

```
pip install pyautd3
```

あるいは, autd3のリポジトリからインストールできる.

```
pip install git+https://github.com/shinolab/autd3.git#subdirectory=python
```

### Linux/macOS

Linux/macOSを使用する場合, 管理者権限が必要な場合がある. その時は, 管理者権限付きでインストールすること.

```
sudo pip install pyautd3
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [チュートリアル](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```python
{{#include ../../../samples/python/main.py}}
```

より詳細なサンプルは[pyautd3のexample](https://github.com/shinolab/autd3/tree/master/python/example)を参照されたい.

## Troubleshooting

Q. linuxやmacから実行できない

A. 管理者権限で実行する

```
sudo python
```

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
