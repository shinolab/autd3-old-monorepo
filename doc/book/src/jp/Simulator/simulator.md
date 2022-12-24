# シミュレータ

AUTDシミュレータはその名の通りAUTD3のシミュレータであり, Windows/Linux/macOSで動作する.

シミュレータを使用するには, 自前でビルドする必要がある.
Windowsの場合は, ビルド済みの実行ファイルが配布されているのでそれを使うこともできる.

## Build

AUTDシミュレータをビルドするには, [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/)をインストールし, CMakeの`BUILD_SIMULATOR`フラグをオンにする必要がある.
或いは, 配布している`simulator`ライブラリをリンクされたい.


```
cmake .. -DBUILD_SIMULATOR=ON
```

AUTDシミュレータの実行に関しては, [サンプル](https://github.com/shinolab/autd3/blob/master/examples/simulator_server.cpp)を参照されたい.

# How to

AUTDシミュレータを実行すると接続待ちの状態になる.
この状態で, `link::Simulator`を使用したクライアントプログラムを実行すると, クライアントプログラムの内容に合わせた音場が表示される.
画面中央の黒いパネルをSliceと呼び, このSliceを使って任意の位置の音場を可視化できる.
また, 振動子の位相が色相で, 振幅が色強度で表される.

なお, シミュレータで表示される音場はシンプルな球面波の重ね合わせであり, 指向性や非線形効果などは考慮されない.

画面左に表示されるGUIでSliceやカメラの操作が行える.
なお, GUIには[Dear ImGui](https://github.com/ocornut/imgui)を用いており, マウスによる操作のほか, "Ctrl+クリック"で数値入力モードになる.

また, GUI以外の場面の"ドラッグ"でカメラの移動, "Shift+ドラッグ"でカメラの回転が行える.

## Slice tab

SliceタブではSliceの大きさと位置, 回転を変えられる.
回転はXYZのオイラー角で指定する.
なお, 「xy」, 「yz」, 「zx」ボタンを押すと, Sliceを各平面に平行な状態に回転させる.

Sliceでは音圧の強さを色で表現する.
Color scaleはこの色空間の音圧の最大値を表す.
大量のデバイスを使用すると色が飽和する場合があるので, その時はColor scaleの値を大きくすれば良い.
また, Sliceそのもののアルファ値をSlice alphaで指定できる.

また, Sliceに表示されている音場の保存ができる.

## Camera tab

Cameraタブではカメラの位置, 回転, Field of View, Near clip, Far clipの設定を変えられる.
回転はXYZのオイラー角で指定する.

## Config tab

Configタブでは音速とフォントサイズ, 及び, 背景色の設定ができる.

また, 各デバイスごとの表示/イネーブルを切り替えられる.
表示をOffにした場合は, 表示されないだけで音場に寄与する.
イネーブルをOffにすると音場に寄与しなくなる.

## Info tab

InfoタブではSilencerやModulation, STMの情報が確認できる.

Silencerの設定は確認できるがこれは音場には反映されない.

また, Modulationも音場には反映されない.
代わりに, 音圧がどのように変調されるかがこのタブで表示される.
また, rawモードではDuty比がどのように変調されるかが表示される.

STMを送信した場合は, STMの情報が表示される.
STMは自動的に切り替わったりしない, 代わりにSTM idxで何番目のデータを表示するかを指定する.

## Other settings

設定の初期値は`simulator::Settings`にて変更できる.

この中で起動時にしか変更できないものとしてvsyncがある.
vsyncをtrueにすると垂直同期が有効になる.
