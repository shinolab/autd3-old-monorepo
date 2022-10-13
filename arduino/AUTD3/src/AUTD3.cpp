// File: AUTD3.cpp
// Project: src
// Created Date: 13/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <AUTD3.h>
#include <Arduino.h>

#if defined(__cplusplus)
extern "C" {
#endif

#define EC_TIMEOUTRET (20000)
#define EC_TIMEOUTSTATE (2000000)
#define EC_MAXSLAVE (64)
#define EC_MAXNAME (40)
#define EC_MAXSM (8)
#define EC_MAXFMMU (4)
#define EC_MAXGROUP (2)
#define EC_MAXIOSEGMENTS (64)

typedef enum {
  EC_STATE_NONE = 0x00,
  EC_STATE_INIT = 0x01,
  EC_STATE_PRE_OP = 0x02,
  EC_STATE_BOOT = 0x03,
  EC_STATE_SAFE_OP = 0x04,
  EC_STATE_OPERATIONAL = 0x08,
  EC_STATE_ACK = 0x10,
  EC_STATE_ERROR = 0x10
} ec_state;

typedef struct __attribute__((packed)) ec_fmmu {
  uint32_t LogStart;
  uint16_t LogLength;
  uint8_t LogStartbit;
  uint8_t LogEndbit;
  uint16_t PhysStart;
  uint8_t PhysStartBit;
  uint8_t FMMUtype;
  uint8_t FMMUactive;
  uint8_t unused1;
  uint16_t unused2;
} ec_fmmut;

typedef struct __attribute__((packed)) ec_sm {
  uint16_t StartAddr;
  uint16_t SMlength;
  uint32_t SMflags;
} ec_smt;

typedef struct ec_slave {
  uint16_t state;
  uint16_t ALstatuscode;
  uint16_t configadr;
  uint16_t aliasadr;
  uint32_t eep_man;
  uint32_t eep_id;
  uint32_t eep_rev;
  uint16_t Itype;
  uint16_t Dtype;
  uint16_t Obits;
  uint32_t Obytes;
  uint8_t* outputs;
  uint8_t Ostartbit;
  uint16_t Ibits;
  uint32_t Ibytes;
  uint8_t* inputs;
  uint8_t Istartbit;
  ec_smt SM[EC_MAXSM];
  uint8_t SMtype[EC_MAXSM];
  ec_fmmut FMMU[EC_MAXFMMU];
  uint8_t FMMU0func;
  uint8_t FMMU1func;
  uint8_t FMMU2func;
  uint8_t FMMU3func;
  uint16_t mbx_l;
  uint16_t mbx_wo;
  uint16_t mbx_rl;
  uint16_t mbx_ro;
  uint16_t mbx_proto;
  uint8_t mbx_cnt;
  boolean hasdc;
  uint8_t ptype;
  uint8_t topology;
  uint8_t activeports;
  uint8_t consumedports;
  uint16_t parent;
  uint8_t parentport;
  uint8_t entryport;
  int32_t DCrtA;
  int32_t DCrtB;
  int32_t DCrtC;
  int32_t DCrtD;
  int32_t pdelay;
  uint16_t DCnext;
  uint16_t DCprevious;
  int32_t DCcycle;
  int32_t DCshift;
  uint8_t DCactive;
  uint16_t configindex;
  uint16_t SIIindex;
  uint8_t eep_8byte;
  uint8_t eep_pdi;
  uint8_t CoEdetails;
  uint8_t FoEdetails;
  uint8_t EoEdetails;
  uint8_t SoEdetails;
  int16_t Ebuscurrent;
  uint8_t blockLRW;
  uint8_t group;
  uint8_t FMMUunused;
  boolean islost;
  int32_t (*PO2SOconfig)(uint16_t slave);
  char name[EC_MAXNAME + 1];
} ec_slavet;

typedef struct ec_group {
  uint32_t logstartaddr;
  uint32_t Obytes;
  uint8_t* outputs;
  uint32_t Ibytes;
  uint8_t* inputs;
  boolean hasdc;
  uint16_t DCnext;
  int16_t Ebuscurrent;
  uint8_t blockLRW;
  uint16_t nsegments;
  uint16_t Isegment;
  uint16_t Ioffset;
  uint16_t outputsWKC;
  uint16_t inputsWKC;
  boolean docheckstate;
  uint32_t IOsegment[EC_MAXIOSEGMENTS];
} ec_groupt;

#define NUM_TRANS_IN_UNIT (249)

#define HEADER_SIZE (128)
#define BODY_SIZE (498)
#define INPUT_SIZE (2)
#define EC_CYCLE_TIME_BASE_NANO_SEC (500 * 1000)

#define MSG_CLEAR (0x00)
#define MSG_RD_CPU_VERSION (0x01)
#define MSG_RD_FPGA_VERSION (0x03)
#define MSG_RD_FPGA_FUNCTION (0x04)
#define MSG_BEGIN (0x05)
#define MSG_END (0xF0)

#define FPGA_CTL_LEGACY_MODE (1 << 0)

#define CPU_CTL_MOD (1 << 0)
#define CPU_CTL_MOD_BEGIN (1 << 1)
#define CPU_CTL_MOD_END (1 << 2)
#define CPU_CTL_WRITE_BODY (1 << 3)

#define MSG_ID_IDX (0)
#define FPGA_CTL_IDX (1)
#define CPU_CTL_IDX (2)
#define MOD_SIZE_IDX (3)
#define MOD_FREQ_DIV_IDX (4)
#define MOD_BEGIN_IDX (8)

#define SYNC0_CYCLE (2)
#define SEND0_CYCLE (2)
#define DEV_NUM (1)

extern int ec_init(const char* ifname);
extern int ec_config_map(void* pIOmap);
int ec_config_init(uint8_t usetable);
extern boolean ec_configdc();
extern int ec_receive_processdata(int32_t timeout);
extern int ec_send_processdata(void);
extern int ec_writestate(uint16_t slave);
extern void ec_dcsync0(uint16_t slave, boolean act, uint32_t CyclTime, int32_t CyclShift);
extern uint16_t ec_statecheck(uint16_t slave, uint16_t reqstate, int32_t timeout);
extern int32_t ec_readstate(void);
extern int64_t ec_DCtime;
extern ec_slavet ec_slave[EC_MAXSLAVE];
extern ec_groupt ec_group[EC_MAXGROUP];
extern int32_t ec_slavecount;

char AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE + INPUT_SIZE) * DEV_NUM];

int64_t ec_sync(const int64_t reftime, const int64_t cycletime, int64_t* integral) {
  int64_t delta = (reftime - 50000) % cycletime;
  if (delta > (cycletime / 2)) delta -= cycletime;
  if (delta > 0) *integral += 1;
  if (delta < 0) *integral -= 1;
  return -(delta / 100) - (*integral / 20);
}

int32_t PO2SO(const uint16_t slave) {
  ec_dcsync0(slave, true, EC_CYCLE_TIME_BASE_NANO_SEC * SYNC0_CYCLE, 0U);
  return 0;
}

AUTD3::AUTD3() { _msg_id = MSG_BEGIN; }

int AUTD3::open() {
  const char ifname[] = "";
  if (ec_init(ifname) <= 0) {
    printf("No socket connection\n");
    return -1;
  }

  const int wc = ec_config_init(0);
  if (wc <= 0) {
    printf("No slaves found\n");
    return -1;
  }

  if (wc != DEV_NUM) {
    printf("The number of slaves you configured: %d, but found: %d\n", DEV_NUM, wc);
    return -1;
  }

  for (int cnt = 1; cnt <= ec_slavecount; cnt++) ec_slave[cnt].PO2SOconfig = PO2SO;

  ec_configdc();

  ec_config_map(AUTD3_IOmap);

  ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE * 4);
  ec_readstate();
  ec_slave[0].state = EC_STATE_OPERATIONAL;
  ec_writestate(0);

  _expected_wkc = ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC;
  _ts = micros() * 1000;

  return 0;
}

void AUTD3::set_gain(char* gain) {
  for (int dev = 0; dev < DEV_NUM; dev++) {
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + MSG_ID_IDX] = get_msg_id();
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + FPGA_CTL_IDX] |= FPGA_CTL_LEGACY_MODE;
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + CPU_CTL_IDX] |= CPU_CTL_WRITE_BODY;
    memcpy(AUTD3_IOmap + (HEADER_SIZE + BODY_SIZE) * dev, gain, BODY_SIZE);
  }
}

void AUTD3::set_modulation(char* mod, int size) {
  for (int dev = 0; dev < DEV_NUM; dev++) {
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + MSG_ID_IDX] = get_msg_id();
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + CPU_CTL_IDX] |= CPU_CTL_MOD | CPU_CTL_MOD_BEGIN | CPU_CTL_MOD_END;
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + MOD_SIZE_IDX] = size;
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + MOD_FREQ_DIV_IDX] = 0x00;
    AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + MOD_FREQ_DIV_IDX + 1] = 0xa0;
    memcpy(AUTD3_IOmap + (HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + MOD_BEGIN_IDX, mod, size);
  }
}

void AUTD3::run() {
  _ts += EC_CYCLE_TIME_BASE_NANO_SEC * SEND0_CYCLE + _toff;

  while ((int64_t)micros() * 1000 < _ts) {
  }

  if (ec_slave[0].state != EC_STATE_OPERATIONAL) {
    printf("EC_STATE changed: {}", ec_slave[0].state);
    ec_slave[0].state = EC_STATE_OPERATIONAL;
    ec_writestate(0);
  }

  ec_send_processdata();
  if (ec_receive_processdata(EC_TIMEOUTRET) != _expected_wkc) return;

  ec_sync(ec_DCtime, EC_CYCLE_TIME_BASE_NANO_SEC * SEND0_CYCLE, &_toff);
}

void AUTD3::stop() {
  for (int dev = 0; dev < DEV_NUM; dev++) AUTD3_IOmap[(HEADER_SIZE + BODY_SIZE) * dev + BODY_SIZE + MSG_ID_IDX] = MSG_CLEAR;
}

char AUTD3::get_msg_id() {
  if (_msg_id >= MSG_END) _msg_id = MSG_BEGIN;
  return _msg_id++;
}

#ifdef __cplusplus
}
#endif
