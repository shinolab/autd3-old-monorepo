# Simulator

autd-simulatorはその名の通りAUTD3のSimulatorであり, Windows/Linux/macOSで動作する.

## Build

AUTD3-Simulatorをビルドするには, Vulkan SDKをインストールし, CMakeの`BUILD_SIMULATOR`フラグをオンにしてビルドする.

```
cmake .. -DBUILD_SIMULATOR=ON
```

# How to

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/emu-home.jpg"/>
  <figcaption>Simulator</figcaption>
</figure>

autd-simulatorを実行すると上図のような画面になる.
この状態で, Simulator linkを使用したクライアントプログラムを実行すると, クライアントプログラムの内容に合わせた音場が表示される.
図の中央の黒いパネルをSliceと呼び, このSliceを使って任意の位置の音場を可視化できる.
また, 振動子の位相が色相で, 振幅が色強度で表される.

なお, シミュレータで表示される音場はシンプルな球面波の重ね合わせであり, 指向性や非線形効果などは考慮されない.

画面左に表示されるGUIでSliceやカメラの操作が行える.
なお, GUIには[Dear ImGui](https://github.com/ocornut/imgui)を用いており, マウスによる操作のほか, "Ctrl+クリック"で数値入力モードになる.

また, GUI以外の場面の"ドラッグ"でカメラの移動, "Shift+ドラッグ"でカメラの回転が行える.

## Slice tab

SliceタブではSliceの大きさと位置, 回転を変えられる.
回転はXYZのオイラー角で指定する.
なお, "xy", "yz", "zx"ボタンを押すと, Sliceを各平面に平行な状態に回転させる.

Sliceでは音圧の強さを色で表現する.
Color scaleはこの色空間の音圧の最大値を表す.
大量のデバイスを使用すると色が飽和する場合があるので, その時はColor scaleの値を大きくすれば良い.
また, Sliceそのもののアルファ値をSlice alphaで指定できる.

Sliceに表示されている音場の保存及び録画ができる.

## Camera tab

Cameraタブではカメラの位置, 回転, Field of Viewの角度, Near clip, Far clipの設定を変えられる.
回転はXYZのオイラー角で指定する.

## Config tab

Configタブでは音速と振動子のアルファ値, 及び, 背景色の設定ができる.

また, 各デバイスごとの表示/イネーブルを切り替えられる.
表示をOffにした場合は, 表示されないだけで音場に寄与する.
イネーブルをOffにすると音場に寄与しなくなる.
さらに, デバイス毎の軸の表示もできる.

## Info tab

InfoタブではSilencerやModulation, STMの情報が確認できる.

Silencerの設定は確認できるがこれは音場には反映されない.

また, Modulationも音場には反映されない.
代わりに, 音圧がどのように変調されるかがこのタブで表示される.
また, rawモードではDuty比がどのように変調されるかが表示される.

STMを送信した場合は, STMの情報が表示される.
STMは自動的に切り替わったりしない, 代わりにSTM idxで何番目のデータを表示するかを指定する.

## Log tab

Logタブではデバッグ用のログが表示される.

## Other settings

すべての設定は`settings.json`に保存される.
幾つかの設定は`settings.json`からのみ編集できる.
この中で重要なものとして, portとvsyncがある.

portはSDKのSimulator linkとの間で使うポート番号である.
また, vsyncをtrueにすると垂直同期が有効になる.
