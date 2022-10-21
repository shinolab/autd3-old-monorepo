# Julia

[AUTD3.jl](https://github.com/shinolab/autd3/tree/master/julia)はJuliaに対応したラッパーを提供している.

## Installation

GitHubのリポジトリからインストールできる.

```
(v1.7) pkg> add https://github.com/shinolab/autd3.git:julia
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```julia
{{#include ../../../samples/julia/sample.jl}}
```

より詳細なサンプルは[AUTD3.jlのexample](https://github.com/shinolab/autd3/tree/master/julia/example)を参照されたい.

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
