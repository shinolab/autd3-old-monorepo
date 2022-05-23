# python

[pyautd](https://github.com/shinolab/pyautd)はpythonに対応したラッパーを提供している.

## Installation

[PyPI](https://pypi.org/project/pyautd3/)で公開しているので, pipからインストールすること.

```
pip install pyautd3
```

あるいは, pyautdのリポジトリからインストールできる.

```
pip install git+https://github.com/shinolab/pyautd.git
```

### Linux/macOS

Linux/macOSを使用する場合, 管理者権限が必要な場合がある. その時は, 管理者権限付きでインストールすること.

```
sudo pip install pyautd3
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```python
from pyautd3 import AUTD, SOEM, Focus, Sine, TRANS_SPACING_MM, NUM_TRANS_X, NUM_TRANS_Y, SilencerConfig


def get_adapter_name():
    adapters = SOEM.enumerate_adapters()
    for i, adapter in enumerate(adapters):
        print('[' + str(i) + ']: ' + adapter[0] + ', ' + adapter[1])

    index = int(input('choose number: '))
    return adapters[index][0]


if __name__ == '__main__':
    autd = AUTD()

    autd.add_device([0., 0., 0.], [0., 0., 0.])

    ifname = get_adapter_name()
    link = SOEM(ifname, autd.num_devices())
    if not autd.open(link):
        print(AUTD.last_error())
        exit()

    autd.check_ack = True

    autd.clear()

    autd.synchronize()

    firm_info_list = autd.firmware_info_list()
    for i, firm in enumerate(firm_info_list):
        print(f'[{i}]: {firm}')

    config = SilencerConfig()
    autd.send(config)

    x = TRANS_SPACING_MM * ((NUM_TRANS_X - 1) / 2.0)
    y = TRANS_SPACING_MM * ((NUM_TRANS_Y - 1) / 2.0)
    z = 150.0
    g = Focus([x, y, z])
    m = Sine(150)
    autd.send(m, g)

    _ = input()

    autd.close()
```

より詳細なサンプルは[pyautdのexample](https://github.com/shinolab/pyautd/tree/master/example)を参照されたい.

## Trouble shooting

Q. linuxやmacから実行できない

A. 管理者権限で実行する

```
sudo python
```

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/pyautd/issues)にてお願いします.
