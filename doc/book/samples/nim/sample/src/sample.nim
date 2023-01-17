import autd3
import autd3/soem

proc onLost(msg: cstring) =
    echo msg
    quit(-1)

when isMainModule:
    try:
        var cnt = initController()
        discard cnt.addDevice([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])

        var soem = initSOEM()
        let link = soem.highPrecision(true).onLost(onLost).build()
        if not cnt.open(link):
            system.quit(-1)

        cnt.ackCheckTimeoutMs = 20

        cnt.send(clear())
        cnt.send(synchronize())

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
