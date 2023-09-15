# Controller

ここでは, `Controller`クラスに存在するAPIを紹介する.

[[_TOC_]]


## fpga_info

FPGAの状態を取得する.
これを使用する前に, `Device`の`reads_fpga_info`フラグをセットしておく必要がある.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).open_with(autd3::link::NullLink {}).unwrap();
autd.geometry_mut()[0].reads_fpga_info = true;
autd.send(UpdateFlags::new())?;

let info = autd.fpga_info();
# Ok(())
# }
```

```cpp
autd.geometry()[0].reads_fpga_info(true);
autd.send(autd3::UpdateFlags());

const auto info = autd.fpga_info();
```

```cs
autd.Geometry[0].ReadsFPGAInfo = true;
autd.Send(new UpdateFlags());

var info = autd.FPGAInfo;
```

```python
autd.geometry[0].reads_fpga_info = True
autd.send(UpdateFlags())

info = autd.fpga_info
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

### Stop

`Stop`を送信すると, 出力を止めることができる.

`Stop`を送信すると, Silencerの設定がリセットされるので注意されたい.

### Clear

`Clear`を送信すると, デバイス内のフラグや`Gain`/`Modulation`データ等をクリアする.

## group

`group`関数を使用すると, デバイスをグルーピングすることができる.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).open_with(autd3::link::NullLink{}).unwrap();
# let x = 0.;
# let y = 0.;
# let z = 0.;
autd.group(|dev| match dev.idx() {
    0 => Some("focus"),
    1 => Some("null"),
    _ => None,
})
.set("null", Null::new())?
.set("focus", Focus::new(Vector3::new(x, y, z)))?
.send()?;
# Ok(())
# }
```

```cpp
autd.group([](const autd3::Device& dev) -> std::optional<const char*> {
    if (dev.idx() == 0) {
        return "null";
    } else if (dev.idx() == 1) {
        return "focus";
    } else {
        return std::nullopt;
    }
    })
    .set("null", autd3::gain::Null())
    .set("focus", autd3::gain::Focus(x, y, z))
    .send();
```

```cs
autd.Group(dev =>
    {
        return dev.Idx switch
        {
            0 => "null",
            1 => "focus",
            _ => null
        };
    })
    .Set("null", new Null())
    .Set("focus", new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150)))
    .Send();
```

```python
def grouping(dev):
    if dev.idx == 0:
        return "null"
    elif dev.idx == 1:
        return "focus"
    else:
        return None

autd.group(grouping)\
    .set("null", Null())\
    .set("focus", Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0])))\
    .send()
```

`gain::Group`とは異なり, 通常の`send`で送信できるデータなら何でも使用できる.
ただし, デバイス単位でしかグルーピングできない.

なお, タイムアウトは, `set`したものの中で最大のものが使用される.
`set`したものの中にタイムアウトを指定したものがなければ, [Link](./link.md)で設定したタイムアウト時間が使用される.

> NOTE:
> このサンプルでは, キーとして文字列を使用しているが, HashMapのキーとして使用できるものなら何でも良い.
