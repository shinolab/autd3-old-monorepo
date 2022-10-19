# Concept

SDKを構成する主なクラスは以下の通りである.

* `Controller` - コントローラクラス. AUTD3に対する全ての操作はこのクラスを介して行う.
* `Geometry` - 現実世界におけるデバイスの配置を管理する
* `Link` - AUTD3デバイスとのインターフェース
* `Gain` - 各振動子の位相/振幅を管理するクラス
* `Modulation` - AM変調を管理するクラス
* `STM` - Hardware上のSpatio-Temporal Modulation (STM) 機能を管理するクラス

SDK使用の流れは次のようになる.

* `Controller`の作成
* 接続されているデバイスの位置と姿勢の設定
* `Link`の作成, 及び, 接続
* デバイスの初期化
* `Gain`, `STM`, `Modulation`の作成, 及び, 送信

## Hardware description

以下にAUTD3を上から見た写真を載せる.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/doc/book/src/fig/Users_Manual/autd_trans_idx.jpg"/>
  <figcaption>AUTD front</figcaption>
</figure>

また, 以下にAUTD3の背面の画像を載せる. 24V電源のコネクタはMolex社5566-02Aを使用している.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/doc/book/src/fig/Users_Manual/autd_back.jpg"/>
  <figcaption>AUTD back</figcaption>
</figure>

AUTD3には一台あたり249個[^fn_asm]の振動子から構成されており, それぞれ図のようにindex番号が振られている.
SDKからはこの全ての振動子の周波数, 位相, 及び, 振幅をそれぞれ個別に指定できるようになっている.

AUTD3の座標系は右手座標系を採用しており, 0番目の振動子の中心が原点になる.
$x$軸は長軸方向, すなわち, 0→17の方向であり, $y$軸は0→18の方向である.
また, 単位系として, 距離は$\SI{}{mm}$, 角度は$\SI{}{rad}$, 周波数は$\SI{}{Hz}$を採用している.
振動子は$\SI{10.16}{mm}$の間隔で配置されており, 基板を含めたサイズは$\SI{192}{mm}\times\SI{151.4}{mm}$となっている.
以下に, 振動子アレイの外形図を載せる.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/doc/book/src/fig/Users_Manual/transducers_array.jpg"/>
  <figcaption>Design drawing of transducer array</figcaption>
</figure>

さらに, AUTD3は復数のデバイスをデイジーチェーンで接続し拡張できるようになっている.
PCと1台目のEherCAT Inをイーサネットケーブルを繋ぎ, $i$台目のEherCAT Outと$i+1$台目のEherCAT Inを繋ぐことで拡張アレイを構成できる.
使用するイーサネットケーブルはCAT 5e以上のものである必要がある.

[^fn_asm]: $18\times 14=252$からネジ用に3つの振動子が抜けている. 態々この位置にネジ穴を持ってきたのは, 複数台並べたときの隙間を可能な限り小さくしようとしたため.
