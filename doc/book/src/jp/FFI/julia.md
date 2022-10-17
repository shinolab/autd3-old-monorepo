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
using Printf

using StaticArrays

using AUTD3

function on_lost(msg::Cstring)
    println(msg)
    exit(-1)
end

const cnt = Controller()
cnt.add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0))

const link = SOEM(on_lost=on_lost, high_precision=true)

if !cnt.open(link)
    println(get_last_error())
    exit(-1)
end

cnt.clear()
cnt.synchronize()

firm_info_list = cnt.firmware_info_list()
for firm_info in firm_info_list
    @printf("%s\n", firm_info)
end

const g = Focus(SVector(90.0, 80.0, 150.0))
const m = Sine(150)

cnt.send(m, g)

readline()

cnt.close()
```

より詳細なサンプルは[AUTD3.jlのexample](https://github.com/shinolab/autd3/tree/master/julia/example)を参照されたい.

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
