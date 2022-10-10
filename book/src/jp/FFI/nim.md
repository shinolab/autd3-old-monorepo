# Nim

[autd3_sim](https://github.com/shinolab/autd3/tree/master/nim)はNimに対応したラッパーを提供している.

## Installation

nimbleでインストールできる.

```
requires "https://github.com/shinolab/autd3.git?subdir=nim == 2.4.1"
```

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```nim
import strformat
import strutils

import autd3
import autd3/soem

import tests/runner

proc onLost(msg: cstring) =
    echo msg
    quit(-1)

when isMainModule:
    try:
        var cnt = initController()
        cnt.addDevice([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])

        var soem = initSOEM()
        let link = soem.highPrecision(true).onLost(onLost).build()
        if not cnt.open(link):
            echo Controller.lastError
            quit(-1)

        cnt.checkTrials = 50

        cnt.clear()
        cnt.synchronize()

        let firmList = cnt.firmwareInfoList()
        for firm in firmList:
            echo firm

        let config = initSilencerConfig()
        cnt.send(config)

        let f = initFocus([90.0, 80.0, 150.0])
        let m = initSine(150)

        cnt.send(m, f)

        discard stdin.readLine

        cnt.close()

    except:
        let
            e = getCurrentException()
            msg = getCurrentExceptionMsg()
        echo "Got exception ", repr(e), " with message ", msg

```

より詳細なサンプルは[example](https://github.com/shinolab/autd3/tree/master/nim/examples)を参照されたい.

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
