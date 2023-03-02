# File: holo.nim
# Project: native_methods
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 10/11/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import backend
import gain
import native_methods/autd3capi_gain_holo

type BackendEigen* = object of Backend

func initBackendEigen*(): BackendEigen =
    AUTDEigenBackend(result.p.addr)

type Constraint* = object of RootObj
    p: pointer

type DontCare* = object of Constraint

func initDontCate*(): DontCare =
    result.p = pointer(nil)
    AUTDConstraintDontCare(result.p.addr)

type Normalize* = object of Constraint

func initNormalize*(): Normalize =
    result.p = pointer(nil)
    AUTDConstraintNormalize(result.p.addr)

type Uniform* = object of Constraint

func initUniform*(value: float64): Uniform =
    result.p = pointer(nil)
    AUTDConstraintUniform(result.p.addr, value)

type Clamp* = object of Constraint

func initClamp*(): Clamp =
    result.p = pointer(nil)
    AUTDConstraintClamp(result.p.addr)

type Holo = object of Gain

func add*(self: Holo, pos: openArray[float64], amp: float64 = 1.0) =
    AUTDGainHoloAdd(self.p, pos[0], pos[1], pos[2], amp)

func `constraint=`*(self: Holo, constraint: Constraint) =
    AUTDSetConstraint(self.p, constraint.p)

type SDP* = object of Holo

func initSDP*(backend: Backend, alpha: float64 = 1e-3, lambda: float64 = 0.9,
        repeat: uint64 = 100): SDP =
    AUTDGainHoloSDP(result.p.addr, backend.p, alpha, lambda, repeat)

type EVP* = object of Holo

func initEVP*(backend: Backend, gamma: float64 = 1.0): EVP =
    AUTDGainHoloEVP(result.p.addr, backend.p, gamma)

type Naive* = object of Holo

func initNaive*(backend: Backend): Naive =
    AUTDGainHoloNaive(result.p.addr, backend.p)

type GS* = object of Holo

func initGS*(backend: Backend, repeat: uint64 = 100): GS =
    AUTDGainHoloGS(result.p.addr, backend.p, repeat)

type GSPAT* = object of Holo

func initGSPAT*(backend: Backend, repeat: uint64 = 100): GSPAT =
    AUTDGainHoloGSPAT(result.p.addr, backend.p, repeat)

type LM* = object of Holo

func initLM*(backend: Backend, eps1: float64 = 1e-8, eps2: float64 = 1e-8, tau: float64 = 1e-3,
                 k_max: uint64 = 5, initial: openArray[float64] = @[]): LM =
    let ip = if initial.len == 0: nil else: unsafeAddr initial[0]
    AUTDGainHoloLM(result.p.addr, backend.p, eps1, eps2, tau, k_max, ip, cast[
            int32](initial.len))

type Greedy* = object of Holo

func initGreedy*(backend: Backend, phaseDiv: int32 = 16): Greedy =
    AUTDGainHoloGreedy(result.p.addr, backend.p, phaseDiv)


type LSSGreedy* = object of Holo

func initLSSGreedy*(backend: Backend, phaseDiv: int32 = 16): LSSGreedy =
    AUTDGainHoloLSSGreedy(result.p.addr, backend.p, phaseDiv)

type APO* = object of Holo

func initAPO*(backend: Backend, eps: float64 = 1e-8, lambda: float64 = 1.0,
                 k_max: int32 = 200, lineSearchMax: int32 = 100): APO =
    AUTDGainHoloAPO(result.p.addr, backend.p, eps, lambda, k_max, lineSearchMax)
