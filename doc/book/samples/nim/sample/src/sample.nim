import autd3
import autd3/soem

proc onLost(msg: cstring) =
    echo msg
    quit(-1)

when isMainModule:
    try:
        var builder = initGeometryBuilder()
        discard builder.addDevice([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])
        var geometry = builder.build()

        var soem = initSOEM()
        let link = soem.highPrecision(true).onLost(onLost).build()

        var autd = openController(geometry, link)

        autd.ackCheckTimeoutMs = 20

        autd.send(clear())
        autd.send(synchronize())

        let firmList = autd.firmwareInfoList()
        for firm in firmList:
            echo firm

        let config = initSilencerConfig()
        autd.send(config)

        let center = autd.geometry.center
        let f = initFocus([center[0], center[1], 150.0])
        let m = initSine(150)

        autd.send(m, f)

        discard stdin.readLine

        autd.close()

    except:
        let
            e = getCurrentException()
            msg = getCurrentExceptionMsg()
        echo "Got exception ", repr(e), " with message ", msg
