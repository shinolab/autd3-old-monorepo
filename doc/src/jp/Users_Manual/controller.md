# Controller

ここでは, `Controller`クラスに存在するAPIを紹介する.

[[_TOC_]]

## force_fan

AUTD3にはファンがついており, Auto, Off, Onの3つのファンモードが有る.
Autoモードでは温度監視ICがICの温度を監視し, 一定温度以上になると自動でファンを起動する.
Offモードではファンは常時オフであり, Onモードでは常時オンになる.

モードの切替は, ファン横のジャンパスイッチで行う. 少しわかりにくいが, 以下の図のようにファン側をショートするとAuto, 真ん中でOff, 右側でOnとなる.

<figure>
  <img src="../fig/Users_Manual/fan.jpg"/>
  <figcaption>AUTDファン制御用のジャンパスイッチ</figcaption>
</figure>

Autoモードの場合は温度が高くなると自動的にファンが起動する.
`force_fan`フラグはこのAutoモードでファンを強制的に起動するためのフラグである.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.force_fan(true);
# Ok(())
# }
```

```cpp
autd.force_fan(true);
```

```cs
autd.ForceFan(true);
```

```python
autd.force_fan(True)
```

実際にフラグが更新されるのは`send`を呼んで, 何らかのデータを送信したときになる.
フラグの更新だけがしたい場合は`UpdateFlags`を送信すれば良い.


```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.force_fan(true);
autd.send(UpdateFlags::new())?;
# Ok(())
# }
```

```cpp
autd.force_fan(true);
autd.send(autd3::UpdateFlags());
```

```cs
autd.ForceFan(true);
autd.Send(new UpdateFlags());
```

```python
autd.force_fan(True)
autd.send(UpdateFlags())
```

## fpga_info

FPGAの状態を取得する.
これを使用する前に, `reads_fpga_info`フラグをセットしておく必要がある.


```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.reads_fpga_info(true);
autd.send(UpdateFlags::new())?;

let info = autd.fpga_info();
# Ok(())
# }
```

```cpp
autd.reads_fpga_info(true);
autd.send(autd3::UpdateFlags());

const auto info = autd.fpga_info();
```

```cs
autd.ReadsFPGAInfo(true);
autd.Send(new UpdateFlags());

var info = autd.FPGAInfo;
```

```python
autd.reads_fpga_info(True)
autd.send(UpdateFlags())

info = autd.fpga_info
```

FPGAの状態としては, 現在以下の情報が取得できる.

- ファン制御用の温度センサがアサートされているかどうか

## send

デバイスにデータを送信する.

### タイムアウト

`with_timeout`でタイムアウト時間を指定できる.
これを省略した場合は[Link](./link.md)で設定したタイムアウト時間が使用される.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
# let m = Static::new();
# let g = Null::new();
autd.send((m, g).with_timeout(std::time::Duration::from_millis(20)))?;
# Ok(())
# }
```

```cpp
autd.send(m, g, std::chrono::milliseconds(20));
```

```cs
autd.Send((m, g), TimeSpan.FromMilliseconds(20));
```

```python
autd.send((m, g), timeout=timedelta(milliseconds=20))
```

タイムアウトの値が0より大きい場合, 送信時に送信データがデバイスで処理されるか, 指定したタイムアウト時間が経過するまで待機する.
送信データがデバイスで処理されたのが確認できた場合に`send`関数は`true`を返し, そうでない場合は`false`を返す.

タイムアウトの値が0の場合, `send`関数はチェックを行わない.

確実にデータを送信したい場合はこれを適当な値に設定しておくことをおすすめする.

### stop

`Stop`を送信すると, 出力を止めることができる.

`Stop`を送信すると, Silencerの設定がリセットされるので注意されたい.

### clear

`Clear`を送信すると, デバイス内のフラグや`Gain`/`Modulation`データ等をクリアする.
