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

        autd.send(clear(), 20*1000*1000)
        autd.send(synchronize(), 20*1000*1000)

        let firmList = autd.firmwareInfoList()
        for firm in firmList:
            echo firm

        let config = initSilencerConfig()
        autd.send(config, 20*1000*1000)

        let center = autd.geometry.center
        let f = initFocus([center[0], center[1], 150.0])
        let m = initSine(150)

        autd.send(m, f, 20*1000*1000)

        discard stdin.readLine

        autd.close()

    except:
        let
            e = getCurrentException()
            msg = getCurrentExceptionMsg()
        echo "Got exception ", repr(e), " with message ", msg
