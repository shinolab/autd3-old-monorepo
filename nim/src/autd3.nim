# File: autd3.nim
# Project: src
# Created Date: 11/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 02/02/2023
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

const NUM_TRANS_IN_UNIT* = 249
const NUM_TRANS_X* = 18
const NUM_TRANS_Y* = 14
const TRANS_SPACING* = 10.16
const DEVICE_WIDTH* = 192.0
const DEVICE_HEIGHT* = 151.4

type AUTDException* = object of CatchableError
 
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

type Transducer* = object
    id: int32
    p: pointer

func initTransducer(id: int32, p: pointer): Transducer = 
    result.id= id
    result.p = p

func id*(self: Transducer) : int32 = 
    self.id

func position*(self: Transducer): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransPosition(self.p, self.id, x.addr, y.addr, z.addr)
    [x, y, z]

func frequency*(self: Transducer) : float64 = 
    AUTDGetTransFrequency(self.p, self.id)

func `frequency=`*(self: Transducer, value: float64)  = 
    AUTDSetTransFrequency(self.p, self.id, value)

func cycle*(self: Transducer) : uint16 = 
    AUTDGetTransCycle(self.p, self.id)

func `cycle=`*(self: Transducer, value: uint16)  = 
    AUTDSetTransCycle(self.p, self.id, value)

func modDelay*(self: Transducer) : uint16 = 
    AUTDGetTransModDelay(self.p, self.id)

func `modDelay=`*(self: Transducer, value: uint16)  = 
    AUTDSetTransModDelay(self.p, self.id, value)

func wavelength*(self: Transducer) : float64 = 
    AUTDGetWavelength(self.p, self.id)

func x_direction*(self: Transducer): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransXDirection(self.p, self.id, x.addr, y.addr, z.addr)
    [x, y, z]

func y_direction*(self: Transducer): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransYDirection(self.p, self.id, x.addr, y.addr, z.addr)
    [x, y, z]

func z_direction*(self: Transducer): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDTransZDirection(self.p, self.id, x.addr, y.addr, z.addr)
    [x, y, z]


type Geometry* = object
    p: pointer

func initGeometry(p: pointer) : Geometry = 
    result.p = p

func soundSpeed*(geometry: Geometry): float64 =
    AUTDGetSoundSpeed(geometry.p)

func `soundSpeed=`*(geometry: var Geometry, c: float64) =
    AUTDSetSoundSpeed(geometry.p, c)

func attenuation*(geometry: Geometry): float64 =
    AUTDGetAttenuation(geometry.p)

func `attenuation=`*(geometry: var Geometry, a: float64) =
    AUTDSetAttenuation(geometry.p, a)

func numTransducers*(geometry: Geometry): int32 =
    AUTDNumTransducers(geometry.p)

func numDevices*(geometry: Geometry): int32 =
    AUTDNumDevices(geometry.p)

func center*(geometry: Geometry): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDGeometryCenter(geometry.p, x.addr, y.addr, z.addr)
    [x, y, z]

func centerOf*(geometry: Geometry, devIdx: int32): array[3, float64] =
    var
        x: float64
        y: float64
        z: float64
    AUTDGeometryCenterOf(geometry.p, devIdx, x.addr, y.addr, z.addr)
    [x, y, z]

iterator iter*(geometry: Geometry) : Transducer =
    var id : int32 = 0
    let numTransducers = geometry.numTransducers
    while id < numTransducers:
        yield initTransducer(id, geometry.p)
        id += 1

proc `=destroy`(geometry: var Geometry) =
    if (geometry.p != pointer(nil)):
        AUTDFreeGeometry(geometry.p)
        geometry.p = pointer(nil)

type GeometryBuilder* = object
    p: pointer

func initGeometryBuilder*() : GeometryBuilder = 
    result.p = pointer(nil)
    AUTDCreateGeometryBuilder(result.p.addr)

func addDevice*(builder: GeometryBuilder, pos: openArray[float64], rot: openArray[
        float64]): GeometryBuilder {.discardable.} =
    discard AUTDAddDevice(builder.p, pos[0], pos[1], pos[2], rot[0], rot[1], rot[2])
    builder

func addDeviceQuaternion*(builder: GeometryBuilder, pos: openArray[float64],
        quaternion: openArray[float64]): GeometryBuilder {.discardable.} =
    discard AUTDAddDeviceQuaternion(builder.p, pos[0], pos[1], pos[2], quaternion[0],
            quaternion[1], quaternion[2], quaternion[3])
    builder

func build*(builder: GeometryBuilder) : Geometry =
    var p = pointer(nil)
    AUTDBuildGeometry(p.addr, builder.p)
    initGeometry(p)

type Controller* = object
    geometry: Geometry
    p: pointer

proc `=destroy`(cnt: var Controller) =
    if (cnt.p != pointer(nil)):
        AUTDFreeController(cnt.p)
        cnt.p = pointer(nil)

func initController(p: pointer, geometry: Geometry): Controller = 
    result.p = p
    result.geometry = geometry

func toLegacy*(cnt: Controller) =
    AUTDSetMode(cnt.p, 0)

func toNormal*(cnt: Controller) =
    AUTDSetMode(cnt.p, 1)

func toNormalPhase*(cnt: Controller) =
    AUTDSetMode(cnt.p, 2)

func openController*(geometry: Geometry, link: Link): Controller {.raises: [AUTDException].}=
    var p = pointer(nil)
    if not AUTDOpenController(p.addr, geometry.p, link.p):
        raise AUTDException.newException("Faile to open controller")
    initController(p, geometry)

func geometry*(cnt: Controller): Geometry =
    cnt.geometry

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

func firmwareInfoList*(cnt: Controller): seq[string] =
    var p = pointer(nil)
    let n = AUTDGetFirmwareInfoListPointer(cnt.p, p.addr)
    var list: seq[string] = @[]
    for i in 0..<n:
        var
            matches_version: bool
            is_latest: bool
        var info = cast[cstring]('\0'.repeat(256))
        AUTDGetFirmwareInfo(p, i, info, matches_version.addr, is_latest.addr)
        list.add($info)
    AUTDFreeFirmwareInfoListPointer(p)
    list

func getFPGAInfo*(cnt: Controller): seq[uint8] =
    let numDevices = cnt.geometry.numDevices
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
