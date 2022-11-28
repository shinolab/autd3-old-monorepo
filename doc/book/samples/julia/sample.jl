using Printf

using StaticArrays

using AUTD3

function on_lost(msg::Cstring)
    println(msg)
    exit(-1)
end

const cnt = Controller()
cnt.add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0))

const link = SOEM(on_lost=on_lost, high_precision=true)
!cnt.open(link)

cnt.set_ack_check_timeout = 20 * 1000 * 1000

cnt.send(Clear())
cnt.send(Synchronize())

firm_info_list = cnt.firmware_info_list()
for firm_info in firm_info_list
    @printf("%s\n", firm_info)
end

const g = Focus(SVector(90.0, 80.0, 150.0))
const m = Sine(150)

cnt.send(m, g)

readline()

cnt.close()