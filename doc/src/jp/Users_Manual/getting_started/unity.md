# Unity版チュートリアル

Unity版のライブラリは座標系がz軸反転の左手系になり, 距離の基本単位がmになっているので注意すること.
また, 浮動小数点数型に`double`ではなく, `float`を使用している.

## インストール

Unity Package Manager経由でインストールする.

1. 「メニューバー」→「Edit」→「Project Settings」から「Package Manager」を開く
1. 「Scoped Registry」にて以下を追加し, 保存する
  - Name    : shinolab
  - URL     : https://registry.npmjs.com
  - Scope(s): com.shinolab
1. 「メニューバー」→「Window」→「Package Manager」を開く
1. 左上の「Packages」ドロップダウンメニューから, 「My Registries」を選択する 
1. 以下のパッケージをインストールする
  1. Windowsの場合,「autd3-unity」
  1. macOSの場合,「autd3-unity-mac」
  1. Linuxの場合,「autd3-unity-linux」

Unity版はサンプルプログラムが付属しているので, そちらを参照されたい.
