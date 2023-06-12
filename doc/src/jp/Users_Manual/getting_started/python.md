# Python版チュートリアル

## pyautd3ライブラリのインストール

```shell
pip install pyautd3
```

次に, `main.py`を作成し, 以下のようにする.
これは単一焦点に$\SI{150}{Hz}$のAM変調をかける場合のソースコードである.

```python,filename=main.py
{{#include ../../../../samples/python/main.py}}
```

そして, これを実行する.

```shell
python main.py
```

## Linux,macOS使用時の注意

Linux, macOSでは, SOEMを使用するのに管理者権限が必要な場合がある.

```shell
sudo python main.py
```
とすること.
