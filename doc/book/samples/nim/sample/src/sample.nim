import autd3
import autd3/soem

proc onLost(msg: cstring) =
    echo msg
    quit(-1)

when isMainModule:
    try:
        var geometry = initGeometryBuilder().addDevice([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]).build()

        var soem = initSOEM()
        let link = soem.highPrecision(true).onLost(onLost).build()

        var autd = openController(geometry, link)

        cnt.ackCheckTimeoutMs = 20

        cnt.send(clear())
        cnt.send(synchronize())

        let firmList = cnt.firmwareInfoList()
        for firm in firmList:
            echo firm

        let config = initSilencerConfig()
        cnt.send(config)

        let f = initFocus(cnt.geometry.center + [0.0, 0.0, 150.0])
        let m = initSine(150)

        cnt.send(m, f)

        discard stdin.readLine

        cnt.close()

    except:
        let
            e = getCurrentException()
            msg = getCurrentExceptionMsg()
        echo "Got exception ", repr(e), " with message ", msg
