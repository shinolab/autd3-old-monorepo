# Controller

ここでは, Controllerクラスに存在するその他の機能を紹介する.

## Check Ack

`check_ack`フラグをOnにすると, デバイスへのデータ送信時に, 送信データがきちんとデバイスで処理されたかどうかを確認するようになる.
```cpp
  autd.check_ack = true;
```
`check_ack`が`true`の場合, デバイスにデータを送信する関数 ([Send functions](#send-functions)) は, 送信データがきちんとデバイスで処理されたかどうかを返すようになる.

信頼性の低いlinkを使用する際はOnにしておくことをおすすめする.
なお, Onにすると[Send functions](#send-functions)の実行時間は増加する.

## Force fan

AUTD3のファン制御はAuto, Off, Onの3つのモードが有る. 
Autoモードでは温度監視ICがICの温度を監視し, 一定温度以上になった際に自動的にファンを起動する. 
Offモードではファンは常時オフであり, Onモードでは常時オンになる.

モードの切替は, ファン横のジャンパスイッチで行う. 少しわかりにくいが, 下図のようにファン側をショートするとAuto, 真ん中でOff, 右側でOnとなる.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/fan.jpg"/>
  <figcaption>AUTD Fan jumper switch</figcaption>
</figure>

Autoモードの場合は温度が高くなると自動的にファンが起動する.
`force_fan`フラグはこのAutoモードでファンを強制的に起動するためのフラグである.
実際にフラグが更新されるのは[Send functions](#send-functions)のどれかを呼び出し後になる.

```cpp
  autd.force_fan = true;
```

## Read FPGA info

`reads_fpga_info`フラグをONにすると, デバイスがFPGAの状態を返すようになる.
実際にフラグが更新されるのは[Send functions](#send-functions)のどれかを呼び出し後になる.

FPGAの状態は`fpga_info`関数で取得できる.
```cpp
  autd.reads_fpga_info = true;
  autd.update_flag();
  const auto fpga_info = autd.read_fpga_info();
```
`fpga_info`の返り値は`FPGAInfo`のデバイス分だけの`vector`である.


## stop

`stop`関数で出力を止めることができる.

## clear

デバイス内のフラグや`Gain`/`Modulation`データ等をクリアする.

## Firmware information

`firmware_infos`関数でFirmwareのバージョン情報を取得できる.

```cpp
 for (auto&& firm_info : autd.firmware_infos()) std::cout << firm_info << std::endl;
```

## Send functions

Send functionsとは, 実際にデバイスにデータを送信する関数の総称である.
これらの関数を呼び出すことで, `force fan`, `reads FPGA info`のフラグが更新される.
また, これらの関数は`check_ack`フラグによって挙動が変わる.
`check_ack`が`true`の場合, これらの関数はデバイスが実際にデータを処理するまで待機する.
特に, `Modulation`/`STM`を送信する際は1フレーム毎に確認が入るので, 処理時間が大きく増加する可能性がある.
また, デバイスがデータを処理したことを確認できなかった場合に`false`を返してくる.
`check_ack`が`false`の場合はデータが処理されたかどうかを確認しない, また, 返り値は常に`true`になる.

送信系の関数の一覧は次のとおりである.

* `update_flag`
* `clear`[^fn_clear]
* `close`
* `stop`
* `send`

[^fn_clear]: フラグもクリアされる
