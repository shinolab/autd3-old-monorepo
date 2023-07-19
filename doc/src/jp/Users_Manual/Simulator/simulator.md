# AUTD3 Simulator

AUTD Simulator (以下, シミュレータ) はその名の通りAUTDのシミュレータであり, Windows/Linux/macOSで動作する.

## AUTD Server

シミュレータは`AUTD Server`に付属している.
[GitHub Releases](https://github.com/shinolab/autd3/releases)にてインストーラを配布しているので, これをダウンロードし, 指示に従ってインストールする.

`AUTD Server`を実行すると, 以下のような画面になるので, `Simulator`タブを開き, `Run`ボタンを押すとシミュレータが起動する.

<figure>
  <img src="../../fig/Users_Manual/autdserver_simulator.jpg"/>
  <figcaption>AUTD Server</figcaption>
</figure>

シミュレータが起動すると接続待ちの状態になる.

<figure>
  <img src="../../fig/sim_waiting.jpg"/>
  <figcaption>接続待ち</figcaption>
</figure>

この状態で, `Simulator`リンクを使って`Controller`を`open`すると, シミュレータ上には, 振動子の位置に円と, 画面中央に黒いパネルが表示される.

<figure>
  <img src="../../fig/sim_init.jpg"/>
  <figcaption>初期状態</figcaption>
</figure>

この黒いパネルを"Slice"と呼び, この"Slice"を使って任意の位置の音場を可視化できる.
また, その時, 振動子の位相が色相で, 振幅が色強度で表される.

<figure>
  <img src="../../fig/sim_focus.jpg"/>
  <figcaption>焦点音場</figcaption>
</figure>

なお, シミュレータで表示される音場はシンプルな球面波の重ね合わせであり, 指向性や非線形効果などは考慮されない.

画面左に表示されているGUIでSliceやカメラの操作が行える.
なお, GUIには[Dear ImGui(https://github.com/ocornut/imgui)を用いており, マウスによる操作のほか, "Ctrl+クリック"で数値入力モードになる.

### Sliceタブ

SliceタブではSliceの大きさと位置, 回転を変えられる.
回転はXYZのオイラー角で指定する.
なお, 「xy」, 「yz」, 「zx」ボタンを押すと, Sliceを各平面に平行な状態へ回転させる.

また, 音圧を表示する「Acoustic」モードか, その2乗の値を表示する「Radiation」モードを選択できる.

また, 「Color settings」の項目ではカラーリングのパレットの変更や, color scale, Slice自体のアルファ値の変更ができる.
大量のデバイスを使用すると色が飽和する場合があるので, その時はColor scaleの値を大きくすれば良い.

### Cameraタブ

Cameraタブではカメラの位置, 回転, Field of View, Near clip, Far clipの設定を変えられる.
回転はXYZのオイラー角で指定する.

### Configタブ

Configタブでは音速とフォントサイズ, 及び, 背景色の設定ができる.

また, 各デバイスごとのshow/enable/overheatの設定を切り替えられる.
showをOffにした場合は, 表示されないだけで音場に寄与する.
enableをOffにすると音場に寄与しなくなる.
また, overheatをOnにすると, 温度センサがアサートされた状態を模擬できる.

### Infoタブ

Infoタブでは, FPSやSilencerやModulation, STMの情報が確認できる.

Silencerの設定は確認できるがこれは音場には反映されない.

ModulationのEnableをOnにすると, Modulationが音場に反映される.
何番目の変調データを適用するかをIndexで指定する.
Auto playをOnにすると自動的にIndexをインクリメントする.

> Note: Auto playによる変調のタイミングは実機のタイミングとは異なる.

また, 音圧がどのように変調されるかがこのタブで表示される.
さらに, rawモードではデューティー比がどのように変調されるかが表示される.

STMを送信した場合は, STMの情報が表示される.
何番目のSTMデータを表示するかをIndexで指定する.
Auto playをOnにすると自動的にIndexをインクリメントする.

> Note: Auto playによる変調のタイミングは実機のタイミングとは異なる.

### その他

「Save image as file」にて, 現在のSliceに表示されている音場を画像で保存できる.

「Auto」ボタンはカメラを自動的に適当な場所に移動させる.
「Reset」は起動時の状態にリセットする.
「Default」はデフォルトの設定にリセットする.

また, 設定は"settings.json"に保存される.
このファイルからしか変更できないものとして`vsync`がある.
`vsync`を`false`にすると垂直同期が無効になる.
