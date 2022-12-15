/*
 * File: app.c
 * Project: src
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#include "app.h"

#include "iodefine.h"
#include "params.h"
#include "utils.h"

#define CPU_VERSION (0x87) /* v2.7 */

#define MOD_BUF_SEGMENT_SIZE_WIDTH (15)
#define MOD_BUF_SEGMENT_SIZE (1 << MOD_BUF_SEGMENT_SIZE_WIDTH)
#define MOD_BUF_SEGMENT_SIZE_MASK (MOD_BUF_SEGMENT_SIZE - 1)

#define FOCUS_STM_BUF_SEGMENT_SIZE_WIDTH (11)
#define FOCUS_STM_BUF_SEGMENT_SIZE (1 << FOCUS_STM_BUF_SEGMENT_SIZE_WIDTH)
#define FOCUS_STM_BUF_SEGMENT_SIZE_MASK (FOCUS_STM_BUF_SEGMENT_SIZE - 1)

#define GAIN_STM_BUF_SEGMENT_SIZE_WIDTH (5)
#define GAIN_STM_BUF_SEGMENT_SIZE (1 << GAIN_STM_BUF_SEGMENT_SIZE_WIDTH)
#define GAIN_STM_BUF_SEGMENT_SIZE_MASK (GAIN_STM_BUF_SEGMENT_SIZE - 1)
#define GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH (6)
#define GAIN_STM_LEGACY_BUF_SEGMENT_SIZE (1 << GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH)
#define GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK (GAIN_STM_LEGACY_BUF_SEGMENT_SIZE - 1)

#define GAIN_DATA_MODE_PHASE_DUTY_FULL (0x0001)
#define GAIN_DATA_MODE_PHASE_FULL (0x0002)
#define GAIN_DATA_MODE_PHASE_HALF (0x0004)

#define MSG_CLEAR (0x00)
#define MSG_RD_CPU_VERSION (0x01)
#define MSG_RD_FPGA_VERSION (0x03)
#define MSG_RD_FPGA_FUNCTION (0x04)
#define MSG_BEGIN (0x05)
#define MSG_END (0xF0)

#define WDT_CNT_MAX (1000)

extern RX_STR0 _sRx0;
extern RX_STR1 _sRx1;
extern TX_STR _sTx;

// fire when ethercat packet arrives
extern void recv_ethercat(void);
// fire once after power on
extern void init_app(void);
// fire periodically with 1ms interval
extern void update(void);

typedef enum {
  LEGACY_MODE = 1 << CTL_REG_LEGACY_MODE_BIT,
  FORCE_FAN = 1 << CTL_REG_FORCE_FAN_BIT,
  OP_MODE = 1 << CTL_REG_OP_MODE_BIT,
  STM_GAIN_MODE = 1 << CTL_REG_STM_GAIN_MODE_BIT,
  READS_FPGA_INFO = 1 << CTL_REG_READS_FPGA_INFO_BIT,
  SYNC = 1 << CTL_REG_SYNC_BIT,
  OP_MODE_FPGA = 1 << CTL_REG_OP_MODE_FPGA_BIT
} FPGAControlFlags;

typedef enum {
  MOD = 1 << 0,
  MOD_BEGIN = 1 << 1,
  MOD_END = 1 << 2,
  CONFIG_EN_N = 1 << 0,
  CONFIG_SILENCER = 1 << 1,
  CONFIG_SYNC = 1 << 2,
  WRITE_BODY = 1 << 3,
  STM_BEGIN = 1 << 4,
  STM_END = 1 << 5,
  IS_DUTY = 1 << 6,
  MOD_DELAY = 1 << 7
} CPUControlFlags;

typedef struct {
  uint8_t msg_id;
  uint8_t fpga_ctl_reg;
  uint8_t cpu_ctl_reg;
  uint8_t size;
  union {
    struct {
      uint32_t freq_div;
      uint8_t data[120];
    } MOD_HEAD;
    struct {
      uint8_t data[124];
    } MOD_BODY;
    struct {
      uint16_t cycle;
      uint16_t step;
      uint8_t _data[120];
    } SILENT;
  } DATA;
} GlobalHeader;

typedef struct {
  union {
    struct {
      uint16_t data[TRANS_NUM];
    } NORMAL;
    struct {
      uint16_t cycle[TRANS_NUM];
    } CYCLE;
    struct {
      uint16_t data[TRANS_NUM];
    } FOCUS_STM_INITIAL;
    struct {
      uint16_t data[TRANS_NUM];
    } FOCUS_STM_SUBSEQUENT;
    struct {
      uint16_t data[TRANS_NUM];
    } GAIN_STM_INITIAL;
    struct {
      uint16_t data[TRANS_NUM];
    } GAIN_STM_SUBSEQUENT;
    struct {
      uint16_t data[TRANS_NUM];
    } MOD_DELAY_DATA;
  } DATA;
} Body;

static volatile uint16_t _ack = 0;
static volatile uint8_t _msg_id = 0;
static volatile bool_t _read_fpga_info;

static volatile uint16_t _cycle[TRANS_NUM];

static volatile uint32_t _mod_cycle = 0;

static volatile uint32_t _stm_write = 0;
static volatile uint32_t _stm_cycle = 0;
static volatile uint16_t _stm_gain_data_mode = GAIN_DATA_MODE_PHASE_DUTY_FULL;

static volatile short _wdt_cnt = WDT_CNT_MAX;

inline static uint64_t get_next_sync0() {
  volatile uint64_t next_sync0 = ECATC.DC_CYC_START_TIME.LONGLONG;
  volatile uint64_t sys_time = ECATC.DC_SYS_TIME.LONGLONG;
  while (next_sync0 < sys_time + 250000) {
    sys_time = ECATC.DC_SYS_TIME.LONGLONG;
    if (sys_time > next_sync0) next_sync0 = ECATC.DC_CYC_START_TIME.LONGLONG;
  }
  return next_sync0;
}

void synchronize(const volatile GlobalHeader* header, const volatile Body* body) {
  const volatile uint16_t* cycle = body->DATA.CYCLE.cycle;
  volatile uint64_t next_sync0;

  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CYCLE_BASE, cycle, TRANS_NUM);

  next_sync0 = get_next_sync0();
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_EC_SYNC_TIME_0, (volatile uint16_t*)&next_sync0, sizeof(uint64_t) >> 1);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, header->fpga_ctl_reg | SYNC);

  memcpy_volatile(_cycle, cycle, TRANS_NUM * sizeof(uint16_t));
}

void write_mod(const volatile GlobalHeader* header) {
  uint32_t freq_div;
  uint16_t* data;
  uint32_t segment_capacity;
  uint32_t write = header->size;

  if ((header->cpu_ctl_reg & MOD_BEGIN) != 0) {
    _mod_cycle = 0;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET, 0);
    freq_div = header->DATA.MOD_HEAD.freq_div;
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);
    data = (uint16_t*)header->DATA.MOD_HEAD.data;
  } else {
    data = (uint16_t*)header->DATA.MOD_BODY.data;
  }

  segment_capacity = (_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) + MOD_BUF_SEGMENT_SIZE - _mod_cycle;
  if (write <= segment_capacity) {
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, (write + 1) >> 1);
    _mod_cycle += write;
  } else {
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, segment_capacity >> 1);
    _mod_cycle += segment_capacity;
    data += segment_capacity;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET, (_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) >> MOD_BUF_SEGMENT_SIZE_WIDTH);
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, (write - segment_capacity + 1) >> 1);
    _mod_cycle += write - segment_capacity;
  }

  if ((header->cpu_ctl_reg & MOD_END) != 0) bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
}

void config_silencer(const volatile GlobalHeader* header) {
  uint16_t step = header->DATA.SILENT.step;
  uint16_t cycle = header->DATA.SILENT.cycle;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, step);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, cycle);
}

static void set_mod_delay(const volatile Body* body) {
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_DELAY_BASE, body->DATA.MOD_DELAY_DATA.data, TRANS_NUM);
}

static void write_normal_op_legacy(const volatile Body* body) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr = get_addr(BRAM_SELECT_NORMAL, 0);
  uint32_t cnt = TRANS_NUM;
  volatile uint16_t* dst = &base[addr];
  const volatile uint16_t* src = body->DATA.NORMAL.data;
  while (cnt--) {
    *dst = *src++;
    dst += 2;
  }
}

static void write_normal_op_raw(const volatile Body* body, bool_t is_duty) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr = get_addr(BRAM_SELECT_NORMAL, 0);
  uint32_t cnt = TRANS_NUM;
  volatile uint16_t* dst = &base[addr] + (is_duty ? 1 : 0);
  const volatile uint16_t* src = body->DATA.NORMAL.data;
  while (cnt-- > 0) {
    *dst = *src++;
    dst += 2;
  }
}

static void write_normal_op(const volatile GlobalHeader* header, const volatile Body* body) {
  if (header->fpga_ctl_reg & LEGACY_MODE) {
    write_normal_op_legacy(body);
  } else {
    write_normal_op_raw(body, (header->cpu_ctl_reg & IS_DUTY) != 0);
  }
}

static void write_focus_stm(const volatile GlobalHeader* header, const volatile Body* body) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr;
  volatile uint16_t* dst;
  const volatile uint16_t* src;
  uint32_t freq_div;
  uint32_t sound_speed;
  uint16_t start_idx;
  uint32_t size, cnt;
  uint32_t segment_capacity;

  if ((header->cpu_ctl_reg & STM_BEGIN) != 0) {
    _stm_write = 0;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);

    size = body->DATA.FOCUS_STM_INITIAL.data[0];
    freq_div = (body->DATA.FOCUS_STM_INITIAL.data[2] << 16) | body->DATA.FOCUS_STM_INITIAL.data[1];
    sound_speed = (body->DATA.FOCUS_STM_INITIAL.data[4] << 16) | body->DATA.FOCUS_STM_INITIAL.data[3];
    start_idx = body->DATA.FOCUS_STM_INITIAL.data[5];

    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SOUND_SPEED_0, (uint16_t*)&sound_speed, sizeof(uint32_t) >> 1);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
    src = body->DATA.FOCUS_STM_INITIAL.data + 6;
  } else {
    size = body->DATA.FOCUS_STM_SUBSEQUENT.data[0];
    src = body->DATA.FOCUS_STM_SUBSEQUENT.data + 1;
  }

  segment_capacity = (_stm_write & ~FOCUS_STM_BUF_SEGMENT_SIZE_MASK) + FOCUS_STM_BUF_SEGMENT_SIZE - _stm_write;
  if (size <= segment_capacity) {
    cnt = size;
    addr = get_addr(BRAM_SELECT_STM, (_stm_write & FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_write += size;
  } else {
    cnt = segment_capacity;
    addr = get_addr(BRAM_SELECT_STM, (_stm_write & FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_write += segment_capacity;

    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET,
               (_stm_write & ~FOCUS_STM_BUF_SEGMENT_SIZE_MASK) >> FOCUS_STM_BUF_SEGMENT_SIZE_WIDTH);

    cnt = size - segment_capacity;
    addr = get_addr(BRAM_SELECT_STM, (_stm_write & FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_write += size - segment_capacity;
  }

  if ((header->cpu_ctl_reg & STM_END) != 0) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_write) - 1);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, header->fpga_ctl_reg | OP_MODE_FPGA);
  }
}

static void write_gain_stm_legacy(const volatile GlobalHeader* header, const volatile Body* body) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr;
  volatile uint16_t* dst;
  const volatile uint16_t* src;
  uint32_t freq_div;
  uint16_t start_idx;
  uint32_t cnt;
  uint16_t phase;

  if ((header->cpu_ctl_reg & STM_BEGIN) != 0) {
    _stm_write = 0;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
    freq_div = (body->DATA.GAIN_STM_INITIAL.data[1] << 16) | body->DATA.GAIN_STM_INITIAL.data[0];
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);
    _stm_gain_data_mode = body->DATA.GAIN_STM_INITIAL.data[2];
    _stm_cycle = body->DATA.GAIN_STM_INITIAL.data[3];
    start_idx = body->DATA.GAIN_STM_INITIAL.data[4];
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
    return;
  }

  src = body->DATA.GAIN_STM_SUBSEQUENT.data;

  addr = get_addr(BRAM_SELECT_STM, (_stm_write & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);

  switch (_stm_gain_data_mode) {
    case GAIN_DATA_MODE_PHASE_DUTY_FULL:
      dst = &base[addr];
      _stm_write += 1;
      cnt = TRANS_NUM;
      while (cnt--) *dst++ = *src++;
      break;
    case GAIN_DATA_MODE_PHASE_FULL:
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) *dst++ = 0xFF00 | ((*src++) & 0x00FF);
      _stm_write += 1;
      src = body->DATA.GAIN_STM_SUBSEQUENT.data;
      addr = get_addr(BRAM_SELECT_STM, (_stm_write & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) *dst++ = 0xFF00 | (((*src++) >> 8) & 0x00FF);
      _stm_write += 1;
      break;
    case GAIN_DATA_MODE_PHASE_HALF:
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) {
        phase = (*src++) & 0x000F;
        *dst++ = 0xFF00 | (phase << 4) | phase;
      }
      _stm_write += 1;

      src = body->DATA.GAIN_STM_SUBSEQUENT.data;
      addr = get_addr(BRAM_SELECT_STM, (_stm_write & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) {
        phase = ((*src++) >> 4) & 0x000F;
        *dst++ = 0xFF00 | (phase << 4) | phase;
      }
      _stm_write += 1;

      src = body->DATA.GAIN_STM_SUBSEQUENT.data;
      addr = get_addr(BRAM_SELECT_STM, (_stm_write & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) {
        phase = ((*src++) >> 8) & 0x000F;
        *dst++ = 0xFF00 | (phase << 4) | phase;
      }
      _stm_write += 1;

      src = body->DATA.GAIN_STM_SUBSEQUENT.data;
      addr = get_addr(BRAM_SELECT_STM, (_stm_write & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) {
        phase = ((*src++) >> 12) & 0x000F;
        *dst++ = 0xFF00 | (phase << 4) | phase;
      }
      _stm_write += 1;
      break;
    default:
      break;
  }

  if ((_stm_write & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) == 0)
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET,
               (_stm_write & ~GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) >> GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH);

  if ((header->cpu_ctl_reg & STM_END) != 0) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, header->fpga_ctl_reg | OP_MODE_FPGA);
  }
}

static void write_gain_stm(const volatile GlobalHeader* header, const volatile Body* body) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr;
  volatile uint16_t* dst;
  const volatile uint16_t* src;
  uint32_t freq_div;
  uint16_t start_idx;
  uint32_t cnt;

  if ((header->cpu_ctl_reg & STM_BEGIN) != 0) {
    _stm_write = 0;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
    freq_div = (body->DATA.GAIN_STM_INITIAL.data[1] << 16) | body->DATA.GAIN_STM_INITIAL.data[0];
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);
    _stm_gain_data_mode = body->DATA.GAIN_STM_INITIAL.data[2];
    _stm_cycle = body->DATA.GAIN_STM_INITIAL.data[3];
    start_idx = body->DATA.GAIN_STM_INITIAL.data[4];
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
    return;
  }

  src = body->DATA.GAIN_STM_SUBSEQUENT.data;

  addr = get_addr(BRAM_SELECT_STM, (_stm_write & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);

  switch (_stm_gain_data_mode) {
    case GAIN_DATA_MODE_PHASE_DUTY_FULL:
      if ((header->cpu_ctl_reg & IS_DUTY) != 0) {
        dst = &base[addr] + 1;
        _stm_write += 1;
      } else {
        dst = &base[addr];
      }
      cnt = TRANS_NUM;
      while (cnt--) {
        *dst = *src++;
        dst += 2;
      }
      break;
    case GAIN_DATA_MODE_PHASE_FULL:
      if ((header->cpu_ctl_reg & IS_DUTY) != 0) break;
      dst = &base[addr];
      cnt = 0;
      while (cnt++ < TRANS_NUM) {
        *dst++ = *src++;
        *dst++ = _cycle[cnt] >> 1;
      }
      _stm_write += 1;
      break;
    case GAIN_DATA_MODE_PHASE_HALF:
      // Not supported in normal mode
      break;
    default:
      break;
  }

  if ((_stm_write & GAIN_STM_BUF_SEGMENT_SIZE_MASK) == 0)
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, (_stm_write & ~GAIN_STM_BUF_SEGMENT_SIZE_MASK) >> GAIN_STM_BUF_SEGMENT_SIZE_WIDTH);

  if ((header->cpu_ctl_reg & STM_END) != 0) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, header->fpga_ctl_reg | OP_MODE_FPGA);
  }
}

static void clear(void) {
  uint32_t freq_div_4k = 40960;

  _read_fpga_info = false;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, 0x0000);

  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, 10);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, 4096);

  _stm_write = 0;
  _stm_cycle = 0;

  _mod_cycle = 2;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, (uint16_t*)&freq_div_4k, sizeof(uint32_t) >> 1);
  bram_write(BRAM_SELECT_MOD, 0, 0x0000);

  bram_set(BRAM_SELECT_NORMAL, 0, 0x0000, TRANS_NUM << 1);
}

inline static uint16_t get_cpu_version(void) { return CPU_VERSION; }
inline static uint16_t get_fpga_version(void) { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_VERSION_NUM); }
inline static uint16_t read_fpga_info(void) { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FPGA_INFO); }

void init_app(void) { clear(); }

void update(void) {
  if (ECATC.AL_STATUS_CODE.WORD == 0x001A) {  // Synchronization error
    if (_wdt_cnt < 0) return;
    if (_wdt_cnt-- == 0) clear();
  } else {
    _wdt_cnt = WDT_CNT_MAX;
  }

  switch (_msg_id) {
    case MSG_RD_CPU_VERSION:
    case MSG_RD_FPGA_VERSION:
    case MSG_RD_FPGA_FUNCTION:
      break;
    default:
      if (_read_fpga_info) _ack = (_ack & 0xFF00) | read_fpga_info();
      break;
  }
  _sTx.ack = _ack;
}

void recv_ethercat(void) {
  GlobalHeader* header = (GlobalHeader*)(_sRx1.data);
  Body* body = (Body*)(_sRx0.data);
  if (header->msg_id == _msg_id) return;
  _msg_id = header->msg_id;
  _ack = ((uint16_t)(header->msg_id)) << 8;
  _read_fpga_info = (header->fpga_ctl_reg & READS_FPGA_INFO) != 0;
  if (_read_fpga_info) _ack = (_ack & 0xFF00) | read_fpga_info();

  switch (_msg_id) {
    case MSG_CLEAR:
      clear();
      break;
    case MSG_RD_CPU_VERSION:
      _ack = (_ack & 0xFF00) | (get_cpu_version() & 0xFF);
      break;
    case MSG_RD_FPGA_VERSION:
      _ack = (_ack & 0xFF00) | (get_fpga_version() & 0xFF);
      break;
    case MSG_RD_FPGA_FUNCTION:
      _ack = (_ack & 0xFF00) | ((get_fpga_version() >> 8) & 0xFF);
      break;
    default:
      if (_msg_id > MSG_END) break;

      if (((header->cpu_ctl_reg & MOD) == 0) && ((header->cpu_ctl_reg & CONFIG_SYNC) != 0)) {
        synchronize(header, body);
        break;
      }

      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, header->fpga_ctl_reg);

      if ((header->cpu_ctl_reg & MOD) != 0)
        write_mod(header);
      else if ((header->cpu_ctl_reg & CONFIG_SILENCER) != 0) {
        config_silencer(header);
      };

      if ((header->cpu_ctl_reg & WRITE_BODY) == 0) break;

      if ((header->cpu_ctl_reg & MOD_DELAY) != 0) {
        set_mod_delay(body);
        break;
      }

      if ((header->fpga_ctl_reg & OP_MODE) == 0) {
        write_normal_op(header, body);
        break;
      }

      if ((header->fpga_ctl_reg & STM_GAIN_MODE) == 0)
        write_focus_stm(header, body);
      else if ((header->fpga_ctl_reg & LEGACY_MODE) == 0)
        write_gain_stm(header, body);
      else
        write_gain_stm_legacy(header, body);
  }
  _sTx.ack = _ack;
}
