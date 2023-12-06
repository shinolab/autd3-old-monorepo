# Controller

ここでは, `Controller`クラスに存在するAPIを紹介する.

[[_TOC_]]

## Force fan
  
AUTD3デバイスにはファンがついており, Auto, Off, Onの3つのファンモードが有る. 

Autoモードでは温度監視ICがICの温度を監視し, 一定温度以上になると自動でファンを起動する.
Offモードではファンは常時オフであり, Onモードでは常時オンになる. 

モードの切替は, ファン横のジャンパスイッチで行う. 少しわかりにくいが, 以下の図のようにファン側をショートするとAuto, 真ん中でOff, 右側でOnとなる.

<figure>
    <img src="../fig/Users_Manual/fan.jpg"/>
    <figcaption>AUTDファン制御用のジャンパスイッチ</figcaption>
</figure>

Autoモードの場合は温度が高くなると自動的にファンが起動する.

Autoモードの場合, `ConfigureForceFan`でファンを強制的に起動できる.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_fan.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_fan.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_fan.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_fan.py}}
```

## fpga_info

FPGAの状態を取得する.
これを使用する前に, `ConfigureReadsFPGAInfo`を送信しておく必要がある.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_0.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_0.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_0.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_0.py}}
```

FPGAの状態としては, 現在以下の情報が取得できる.

- ファン制御用の温度センサがアサートされているかどうか

## send

デバイスにデータを送信する.

データは単体か2つのみ同時に送信することができる.
ただし, `Stop`のみ例外で, 単体でしか送信できない.

### タイムアウト

`with_timeout`でタイムアウト時間を指定できる.
これを省略した場合は[Link](./link.md)で設定したタイムアウト時間が使用される.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_1.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_1.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_1.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_1.py}}
```

タイムアウトの値が0より大きい場合, 送信時に送信データがデバイスで処理されるか, 指定したタイムアウト時間が経過するまで待機する.
送信データがデバイスで処理されたのが確認できた場合に`send`関数は`true`を返し, そうでない場合は`false`を返す.

タイムアウトの値が0の場合, `send`関数はチェックを行わない.

確実にデータを送信したい場合はこれを適当な値に設定しておくことをおすすめする.

### Stop

`Stop`を送信すると, 出力を止めることができる.

`Stop`を送信すると, Silencerの設定がリセットされるので注意されたい.

### Clear

`Clear`を送信すると, デバイス内のフラグや`Gain`/`Modulation`データ等をクリアする.

## group

`group`関数を使用すると, デバイスをグルーピングすることができる.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_2.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_2.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_2.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_2.py}}
```

`gain::Group`とは異なり, 通常の`send`で送信できるデータなら何でも使用できる.
ただし, デバイス単位でしかグルーピングできない.

なお, タイムアウトは, `set`したものの中で最大のものが使用される.
`set`したものの中にタイムアウトを指定したものがなければ, [Link](./link.md)で設定したタイムアウト時間が使用される.

> NOTE:
> このサンプルでは, キーとして文字列を使用しているが, HashMapのキーとして使用できるものなら何でも良い.
