# コンセプト

SDKを構成する主なコンポーネントは以下の通りである.

* `Controller` - コントローラクラス. AUTD3に対する全ての操作はこのクラスを介して行う
* `Geometry` - `Device`のコンテナ
  * `Device` - AUTD3デバイスに対応するクラス
* `Link` - クライアントとデバイスとのインターフェース
* `Gain` - 各振動子の位相/振幅を管理するクラス
* `Modulation` - AM変調を管理するクラス
* `STM` - ファームウェア上のSpatio-Temporal Modulation (STM, 時空間変調) 機能を管理するクラス

以下にAUTD3の前面と背面写真を載せる.

<figure>
  <img src="../fig/Users_Manual/autd_trans_idx.jpg"/>
  <figcaption>AUTDの表面写真</figcaption>
</figure>

<figure>
  <img src="../fig/Users_Manual/autd_back.jpg"/>
  <figcaption>AUTD背面写真</figcaption>
</figure>


AUTD3は一台あたり249個の振動子から構成されている[^fn_asm].
SDKからはこの全ての振動子の位相/振幅をそれぞれ個別に指定できるようになっている.
AUTD3の座標系は右手座標系を採用しており, 0番目の振動子の中心が原点になる.
x軸は長軸方向, すなわち, 0→17の方向であり, y軸は0→18の方向である.
また, 単位系として, 距離はmm, 角度はrad, 周波数はHzを採用している.
振動子は$\SI{10.16}{mm}$の間隔で配置されており, 基板を含めたサイズは$\SI{192}{mm}\times\SI{151.4}{mm}$となっている.

以下に振動子アレイの寸法を載せる.

<figure>
  <img src="../fig/Users_Manual/transducers_array.jpg"/>
  <figcaption>AUTD3デバイスの寸法</figcaption>
</figure>

さらに, AUTD3は復数のデバイスをデイジーチェインで接続し拡張できるようになっている.
PCと1台目のEherCAT In をイーサネットケーブルを繋ぎ, $i$台目のEherCAT Outと$i+1$台目のEherCAT Inを繋ぐことで拡張アレイを構成できる.
この時, イーサネットケーブルはCAT 5e以上のものを使用すること.

AUTD3の電源は$\SI{24}{V}$の直流電源を使用する. 電源についても相互に接続でき, 電源コネクタは3つの内で好きなところを使って良い.
なお, 電源のコネクタはMolex社5566-02Aを使用している.

> NOTE: AUTD3は最大でデバイスあたり$\SI{2}{A}$の電流を消費する. 電源の最大出力電流に注意されたい.

[^fn_asm]: $18\times 14=252$からネジ用に3つの振動子が抜けている. 態々この位置にネジ穴を持ってきたのは, 複数台並べたときの隙間を可能な限り小さくしようとしたため.
