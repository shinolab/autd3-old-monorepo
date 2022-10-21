# Nim

[autd3_sim](https://github.com/shinolab/autd3/tree/master/nim) provides a wrapper for Nim.

## Installation

You can install it with nimble.

```
requires "https://github.com/shinolab/autd3.git?subdir=nim == 2.4.4"
```

## Usage

The wrapper is designed to be the same as the C++ version.

For example, the following code is equivalent to [Getting Started](../Users_Manual/getting_started.md).


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

For a more detailed example, see [example](https://github.com/shinolab/autd3/tree/master/nim/examples).

If you have any other questions, please send them to [GitHub issue](https://github.com/shinolab/autd3/issues).
