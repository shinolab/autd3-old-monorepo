# Controller

ここでは, `Controller`クラスに存在するAPIを紹介する.

[[_TOC_]]

## open/close/is_open

`Controller`をopen/closeする.

Controllerがopenしているかどうかは`is_open`で取得できる.

## geometry

`Geometry`を取得する.

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

```cpp
autd.force_fan(true);
```

実際にフラグが更新されるのは`send`を呼んで, 何らかのデータを送信したときになる.
フラグの更新だけがしたい場合は`UpdateFlag`を送信すれば良い.

```cpp
autd.force_fan(true);
autd.send(autd3::UpdateFlag());
```

## fpga_info

FPGAの状態を取得する.
これを使用する前に, `reads_fpga_info`フラグをセットしておく必要がある.

```cpp
autd.reads_fpga_info(true);
autd.send(autd3::update_flag());

const auto infos = autd.fpga_info();
```

FPGAの状態としては, 現在以下の情報が取得できる.

- ファン制御用の温度センサがアサートされているかどうか

## firmware_infos

ファームウェアのバージョン情報を取得する.

## send

デバイスにデータを送信する.

### タイムアウト

`send`の最終引数でタイムアウト時間を指定できる.
この引数を省略した場合は[Link](./link.md)で設定されたデフォルトタイムアウト時間が使用される.
タイムアウト時間の設定がない[Link](./link.md)のデフォルトタイムアウト時間は0である.

```cpp
autd.send(..., autd3::Milliseconds(20));
```

タイムアウトの値が0より大きい場合, 送信時に送信データがデバイスで処理されるか, 指定したタイムアウト時間が経過するまで待機する.
送信データがデバイスで処理されたのが確認できた場合に`send`関数は`true`を返し, そうでない場合は`false`を返す.

タイムアウトの値が0の場合, `send`関数はチェックを行わない.

確実にデータを送信したい場合はこれを適当な値に設定しておくことをおすすめする.

### stop

`autd3::Stop`を送信すると, 出力を止めることができる.

```cpp
autd.send(autd3::Stop());
```

`autd3::Stop`を送信すると, `SilencerConfig`がデフォルトの値で上書きされるので注意されたい.

### clear

`autd3::Clear`を送信すると, デバイス内のフラグや`Gain`/`Modulation`データ等をクリアする.

```cpp
autd.send(autd3::Clear());
```
