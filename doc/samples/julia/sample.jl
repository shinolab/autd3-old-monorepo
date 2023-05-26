using Printf

using StaticArrays

using AUTD3

function on_lost(msg::Cstring)
    println(msg)
    exit(-1)
end

geometry = GeometryBuilder().add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0)).build()

link = SOEM().on_lost(on_lost).build()

cnt = Controller(geometry, link)

cnt.send(Clear())
cnt.send(Synchronize())

firm_info_list = cnt.firmware_info_list()
for firm_info in firm_info_list
    @printf("%s\n", firm_info)
end

const g = Focus(cnt.geometry().center() + SVector(0.0, 0.0, 150.0))
const m = Sine(150)

cnt.send(m, g)

readline()

cnt.close()