# File: autd3.nim
# Project: src
# Created Date: 11/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 24/01/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#

import strutils

import autd3/native_methods/autd3capi
import autd3/link
import autd3/header
import autd3/body
import autd3/gain
import autd3/modulation

type SpecialData* = object of RootObj
    p*: pointer

proc `=destroy`(data: var SpecialData) =
    if (data.p != nil):
        AUTDDeleteSpecialData(data.p)
        data.p = pointer(nil)

func clear*(): SpecialData =
    result.p = pointer(nil)
    AUTDClear(result.p.addr)

func synchronize*(): SpecialData =
    result.p = pointer(nil)
    AUTDSynchronize(result.p.addr)

func update_flag*(): SpecialData =
    result.p = pointer(nil)
    AUTDUpdateFlags(result.p.addr)

func stop*(): SpecialData =
    result.p = pointer(nil)
    AUTDStop(result.p.addr)

func mod_delay_config*(): SpecialData =
    result.p = pointer(nil)
    AUTDModDelayConfig(result.p.addr)

type Controller* = object
    p*: pointer

func initController*(): Controller =
    result.p = pointer(nil)
    AUTDCreateController(result.p.addr)

func toLegacy*(cnt: Controller) =
    AUTDSetMode(cnt.p, 0)

func toNormal*(cnt: Controller) =
    AUTDSetMode(cnt.p, 1)

func toNormalPhase*(cnt: Controller) =
    AUTDSetMode(cnt.p, 2)

func addDevice*(cnt: Controller, pos: openArray[float64], rot: openArray[
        float64]): bool {.discardable.} =
    AUTDAddDevice(cnt.p, pos[0], pos[1], pos[2], rot[0], rot[1], rot[2])

func addDeviceQuaternion*(cnt: Controller, pos: openArray[float64],
        quaternion: openArray[float64]): bool {.discardable.} =
    AUTDAddDeviceQuaternion(cnt.p, pos[0], pos[1], pos[2], quaternion[0],
            quaternion[1], quaternion[2], quaternion[3])

func open*(cnt: Controller, link: Link): bool =
    AUTDOpenController(cnt.p, link.p)

func close*(cnt: Controller): bool {.discardable.} =
    AUTDClose(cnt.p)

func isOpen*(cnt: Controller): bool =
    AUTDIsOpen(cnt.p)

func forceFan*(cnt: Controller): bool =
    AUTDGetForceFan(cnt.p)

func `forceFan=`*(cnt: var Controller, force: bool) =
    AUTDSetForceFan(cnt.p, force)

func readsFPGAInfo*(cnt: Controller): bool =
    AUTDGetReadsFPGAInfo(cnt.p)

func `readsFPGAInfo=`*(cnt: var Controller, flag: bool) =
    AUTDSetReadsFPGAInfo(cnt.p, flag)

func ackCheckTimeoutNs*(cnt: Controller): uint64 =
    AUTDGetAckCheckTimeout(cnt.p)

func `ackCheckTimeoutNs=`*(cnt: var Controller, value: uint64) =
    AUTDSetAckCheckTimeout(cnt.p, value)

func ackCheckTimeoutMs*(cnt: Controller): uint64 =
    AUTDGetAckCheckTimeout(cnt.p) div 1000000

func `ackCheckTimeoutMs=`*(cnt: var Controller, value: uint64) =
    AUTDSetAckCheckTimeout(cnt.p, value * 1000000)

func sendIntervalNs*(cnt: Controller): uint64 =
    AUTDGetSendInterval(cnt.p)

func `sendIntervalNs=`*(cnt: var Controller, value: uint64) =
    AUTDSetSendInterval(cnt.p, value)

func sendIntervalMs*(cnt: Controller): uint64 =
    AUTDGetSendInterval(cnt.p) div 1000000

func `sendIntervalMs=`*(cnt: var Controller, value: uint64) =
    AUTDSetSendInterval(cnt.p, value * 1000000)

func soundSpeed*(cnt: Controller): float64 =
    AUTDGetSoundSpeed(cnt.p)

func `soundSpeed=`*(cnt: var Controller, c: float64) =
    AUTDSetSoundSpeed(cnt.p, c)

func attenuation*(cnt: Controller): float64 =
    AUTDGetAttenuation(cnt.p)

func `attenuation=`*(cnt: var Controller, a: float64) =
    AUTDSetAttenuation(cnt.p, a)

func getTransFrequency*(cnt: Controller,
        transIdx: int32): float64 =
    AUTDGetTransFrequency(cnt.p, transIdx)

func setTransFrequency*(cnt: Controller, transIdx: int32,
        freq: float64) =
    AUTDSetTransFrequency(cnt.p, transIdx, freq)

func getTransCycle*(cnt: Controller,
        transIdx: int32): uint16 =
    AUTDGetTransCycle(cnt.p, transIdx)

func setTransCycle*(cnt: Controller, transIdx: int32,
        cycle: uint16) =
    AUTDSetTransCycle(cnt.p, transIdx, cycle)

func setModDelay*(cnt: Controller, transIdx: int32,
        delay: uint16) =
    AUTDSetTransCycle(cnt.p, transIdx, delay)

func transPosition*(cnt: Controller, transIdx: int32): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransPosition(cnt.p, transIdx, x.addr, y.addr, z.addr)
    [x, y, z]

func transDirectionX*(cnt: Controller, transIdx: int32): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransXDirection(cnt.p, transIdx, x.addr, y.addr, z.addr)
    [x, y, z]

func transDirectionY*(cnt: Controller, transIdx: int32): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransYDirection(cnt.p, transIdx, x.addr, y.addr, z.addr)
    [x, y, z]

func transDirectionZ*(cnt: Controller, transIdx: int32): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransZDirection(cnt.p, transIdx, x.addr, y.addr, z.addr)
    [x, y, z]

func firmwareInfoList*(cnt: Controller): seq[string] =
    var p = pointer(nil)
    let n = AUTDGetFirmwareInfoListPointer(cnt.p, p.addr)
    var list: seq[string] = @[]
    for i in 0..<n:
        var info = cast[cstring]('\0'.repeat(256))
        AUTDGetFirmwareInfo(p, i, info)
        list.add($info)
    AUTDFreeFirmwareInfoListPointer(p)
    list

func wavelength*(cnt: Controller, transIdx: int32): float64 =
    AUTDGetWavelength(cnt.p, transIdx)

func numTransducers*(cnt: Controller): int32 =
    AUTDNumTransducers(cnt.p)

func numDevices*(cnt: Controller): int32 =
    AUTDNumDevices(cnt.p)

func getFPGAInfo*(cnt: Controller): seq[uint8] =
    let numDevices = cnt.numDevices
    var info = newSeq[uint8](numDevices)
    discard AUTDGetFPGAInfo(cnt.p, addr info[0])
    info

func send*(cnt: Controller, header: Header): bool {.discardable.} =
    AUTDSend(cnt.p, header.p, pointer(nil))

func send*(cnt: Controller, header: Header, body: Body): bool {.discardable.} =
    AUTDSend(cnt.p, header.p, body.p)

func send*(cnt: Controller, body: Body): bool {.discardable.} =
    AUTDSend(cnt.p, pointer(nil), body.p)

func send*(cnt: Controller, data: SpecialData): bool {.discardable.} =
    AUTDSendSpecial(cnt.p, data.p)

proc `=destroy`(cnt: var Controller) =
    if (cnt.p != pointer(nil)):
        AUTDFreeController(cnt.p)
        cnt.p = pointer(nil)

type Null* = object of Gain

func initNull*(): Null =
    AUTDGainNull(result.p.addr)

type Grouped* = object of Gain
    gains: seq[Gain]

func initGrouped*(): Grouped =
    AUTDGainGrouped(result.p.addr)
    result.gains = @[]

func add*(self: var Grouped, devId: int32, gain: Gain) =
    AUTDGainGroupedAdd(self.p, devId, gain.p)
    self.gains.add(gain)

type Focus* = object of Gain

func initFocus*(pos: openArray[float64], amp: float64 = 1.0): Focus =
    AUTDGainFocus(result.p.addr, pos[0], pos[1], pos[2], amp)

type BesselBeam* = object of Gain

func initBesselBeam*(apex: openArray[float64], dir: openArray[float64],
        theta: float64, amp: float64 = 1.0): BesselBeam =
    AUTDGainBesselBeam(result.p.addr, apex[0], apex[1], apex[2], dir[0], dir[1],
            dir[2], theta, amp)

type PlaneWave* = object of Gain

func initPlaneWave*(dir: openArray[float64], amp: float64 = 1.0): BesselBeam =
    AUTDGainPlaneWave(result.p.addr, dir[0], dir[1], dir[2], amp)

type CustomGain* = object of Gain

func initCustomGain*(amps: openArray[float64], phases: openArray[
        float64]): CustomGain =
    let n = cast[uint64](amps.len)
    AUTDGainCustom(result.p.addr, unsafeAddr amps[0], unsafeAddr phases[0], n)

type Static* = object of Modulation

func initStatic*(amp: float64 = 1.0): Static =
    AUTDModulationStatic(result.p.addr, amp)

type Sine* = object of Modulation

func initSine*(freq: int32, amp: float64 = 1.0, offset: float64 = 0.5): Sine =
    AUTDModulationSine(result.p.addr, freq, amp, offset)

type SineSquared* = object of Modulation

func initSineSquared*(freq: int32, amp: float64 = 1.0,
        offset: float64 = 0.5): Sine =
    AUTDModulationSineSquared(result.p.addr, freq, amp, offset)

type SineLegacy* = object of Modulation

func initSineLegacy*(freq: float64, amp: float64 = 1.0,
        offset: float64 = 0.5): Sine =
    AUTDModulationSineLegacy(result.p.addr, freq, amp, offset)

type Square* = object of Modulation

func initSquare*(freq: int32, low: float64 = 0.0, high: float64 = 1.0,
        duty: float64 = 0.5): Sine =
    AUTDModulationSquare(result.p.addr, freq, low, high, duty)

type SilencerConfig* = object of Header

type CustomModulation* = object of Modulation

func initCustomModulation*(buf: openArray[float64],
        freqDiv: uint32): CustomModulation =
    let n = cast[uint64](buf.len)
    AUTDModulationCustom(result.p.addr, unsafeAddr buf[0], n, freqDiv)

func initSilencerConfig*(step: uint16 = 10,
        cycle: uint16 = 4096): SilencerConfig =
    AUTDCreateSilencer(result.p.addr, step, cycle)

func none*(_: typedesc[SilencerConfig]): SilencerConfig =
    initSilencerConfig(0xFFFF, 4096)

proc `=destroy`(config: var SilencerConfig) =
    if (config.p != nil):
        AUTDDeleteSilencer(config.p)
        config.p = pointer(nil)

type STM* = object of Body

proc `=destroy`(stm: var STM) =
    if (stm.p != nil):
        AUTDDeleteSTM(stm.p)
        stm.p = pointer(nil)

proc samplingFrequencyDivision*(stm: STM): uint32 =
    AUTDSTMSamplingFrequencyDivision(stm.p)

proc `samplingFrequencyDivision=`*(stm: STM, value: uint32) =
    AUTDSTMSetSamplingFrequencyDivision(stm.p, value)

proc samplingFrequency*(stm: STM): float64 =
    AUTDSTMSamplingFrequency(stm.p)

proc frequency*(stm: STM): float64 =
    AUTDSTMFrequency(stm.p)

proc `frequency=`*(stm: STM, value: float64): float64 {.discardable.} =
    AUTDSTMSetFrequency(stm.p, value)

type FocusSTM* = object of STM

func initFocusSTM*(): FocusSTM =
    AUTDFocusSTM(result.p.addr)

func add*(stm: FocusSTM, pos: openArray[float64],
        shift: uint8 = 0) =
    AUTDFocusSTMAdd(stm.p, pos[0], pos[1], pos[2], shift)

type GainSTM* = object of STM
    gains: seq[Gain]

type Mode* {.pure.} = enum
    PhaseDutyFull = 0x0001
    PhaseFull = 0x0002
    PhaseHalf = 0x0004

func initGainSTM*(): GainSTM =
    AUTDGainSTM(result.p.addr)
    result.gains = @[]

func add*(stm: var GainSTM, gain: Gain) =
    AUTDGainSTMAdd(stm.p, gain.p)
    stm.gains.add(gain)

func mode*(stm: GainSTM): Mode =
    let m = AUTDGetGainSTMMode(stm.p)
    cast[Mode](m)

func `mode=`*(stm: GainSTM, mode: Mode) =
    let m = cast[uint16](ord(mode))
    AUTDSetGainSTMMode(stm.p, m)

type Amplitudes* = object of Body

func initAmplitudes*(amp: float64 = 1.0): Amplitudes =
    AUTDCreateAmplitudes(result.p.addr, amp)

proc `=destroy`(amps: var Amplitudes) =
    if (amps.p != nil):
        AUTDDeleteAmplitudes(amps.p)
        amps.p = pointer(nil)
