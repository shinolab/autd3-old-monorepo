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
1. OSに応じて以下のパッケージをインストールする
    - Windows: `autd3-unity`
    - macOS: `autd3-unity-mac`
    - Linux: `autd3-unity-linux`

Unity版はサンプルプログラムが付属しているので, そちらを参照されたい.
