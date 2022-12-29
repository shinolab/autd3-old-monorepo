# Nim

[autd3_sim](https://github.com/shinolab/autd3/tree/master/nim)はNimに対応したラッパーを提供している.

## Installation

nimbleでインストールできる.

```
requires "https://github.com/shinolab/autd3.git?subdir=nim == 2.7.3"
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [チュートリアル](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```nim
{{#include ../../../samples/nim/sample/src/sample.nim}}
```

より詳細なサンプルは[example](https://github.com/shinolab/autd3/tree/master/nim/examples)を参照されたい.

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
