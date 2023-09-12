/*
 * File: app.c
 * Project: src
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

#include "app.h"

#include "iodefine.h"
#include "params.h"
#include "utils.h"

#define CPU_VERSION_MAJOR (0x8A) /* v3.0 */
#define CPU_VERSION_MINOR (0x01)

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

#define WDT_CNT_MAX (1000)

extern TX_STR _sTx;

#define BUF_SIZE (64)
static volatile RX_STR _buf[BUF_SIZE];
volatile uint8_t _write_cursor;
volatile uint8_t _read_cursor;

static volatile RX_STR _data;

inline static void word_cpy(uint16_t* dst, uint16_t* src, uint32_t cnt) {
  while (cnt-- > 0) *dst++ = *src++;
}

bool_t push(const volatile uint16_t* p_data) {
  uint32_t next;
  next = _write_cursor + 1;

  if (next >= BUF_SIZE) next = 0;

  if (next == _read_cursor) return false;

  word_cpy((uint16_t*)&_buf[_write_cursor], (uint16_t*)p_data, 249);
  word_cpy(((uint16_t*)&_buf[_write_cursor]) + 249, (uint16_t*)&p_data[249 + 1], 64);

  _write_cursor = next;

  return true;
}

bool_t pop(volatile RX_STR* p_data) {
  uint32_t next;

  if (_read_cursor == _write_cursor) return false;

  memcpy_volatile(p_data, &_buf[_read_cursor], sizeof(RX_STR));

  next = _read_cursor + 1;
  if (next >= BUF_SIZE) next = 0;

  _read_cursor = next;

  return true;
}

// fire when ethercat packet arrives
extern void recv_ethercat(uint16_t* p_data);
// fire once after power on
extern void init_app(void);
// fire periodically with 1ms interval
extern void update(void);

typedef enum {
  FORCE_FAN = 1 << CTL_FLAG_FORCE_FAN_BIT,
  READS_FPGA_INFO = 1 << CTL_FLAG_READS_FPGA_INFO_BIT,
} FPGAControlFlags;

typedef struct {
  uint8_t msg_id;
  uint8_t fpga_ctl_flag;
  uint16_t slot_2_offset;
} Header;

static volatile uint8_t _ack = 0;
static volatile uint8_t _rx_data = 0;
static volatile bool_t _read_fpga_info;

static volatile uint16_t _cycle[TRANS_NUM];

static volatile uint32_t _mod_cycle = 0;

static volatile uint32_t _stm_cycle = 0;
static volatile uint16_t _gain_stm_mode = GAIN_STM_MODE_DUTY_PHASE_FULL;

static volatile uint8_t _fpga_flags = 0;
static volatile uint16_t _fpga_flags_internal = 0;

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

void synchronize(const volatile uint8_t* p_data) {
  const volatile uint16_t* cycle = (const volatile uint16_t*)p_data;
  volatile uint64_t next_sync0;
  volatile uint16_t flag;

  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CYCLE_BASE, cycle, TRANS_NUM);

  next_sync0 = get_next_sync0();
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_EC_SYNC_TIME_0, (volatile uint16_t*)&next_sync0, sizeof(uint64_t) >> 1);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG, _fpga_flags_internal | _fpga_flags | CTL_FLAG_SYNC);

  memcpy_volatile(_cycle, cycle, TRANS_NUM * sizeof(uint16_t));

  while (true) {
    flag = bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG);
    if ((flag & CTL_FLAG_SYNC) == 0) break;
  }
}

inline static void change_mod_segment(uint16_t segment) {
  asm("dmb");
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_MEM_SEGMENT, segment);
  asm("dmb");
}

void write_mod(const volatile uint8_t* p_data) {
  uint32_t segment_capacity;
  uint32_t freq_div;

  uint8_t flag = p_data[1];

  uint16_t write = (((uint16_t)p_data[3]) << 8) | (uint16_t)p_data[2];

  const uint16_t* data;
  if ((flag & MODULATION_FLAG_BEGIN) == MODULATION_FLAG_BEGIN) {
    _mod_cycle = 0;
    change_mod_segment(0);
    freq_div = *((const uint32_t*)&p_data[4]);
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);
    data = (const uint16_t*)(&p_data[8]);
  } else {
    data = (const uint16_t*)(&p_data[4]);
  }

  segment_capacity = (_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) + MOD_BUF_SEGMENT_SIZE - _mod_cycle;
  if (write <= segment_capacity) {
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, (write + 1) >> 1);
    _mod_cycle += write;
  } else {
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, segment_capacity >> 1);
    _mod_cycle += segment_capacity;
    data += segment_capacity;
    change_mod_segment((_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) >> MOD_BUF_SEGMENT_SIZE_WIDTH);
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, (write - segment_capacity + 1) >> 1);
    _mod_cycle += write - segment_capacity;
  }

  if ((flag & MODULATION_FLAG_END) == MODULATION_FLAG_END) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
  }
}

void config_silencer(const volatile uint8_t* p_data) {
  uint16_t step = *((const uint16_t*)&p_data[0]);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, step);
}

static void write_mod_delay(const volatile uint8_t* p_data) {
  const uint16_t* delay = (const uint16_t*)p_data;
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_DELAY_BASE, delay, TRANS_NUM);
}

inline static void write_duty_filter(const volatile uint8_t* p_data) {
  const uint16_t* filter = (const uint16_t*)p_data;
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FILTER_DUTY_BASE, filter, TRANS_NUM);
}

inline static void write_phase_filter(const volatile uint8_t* p_data) {
  const uint16_t* filter = (const uint16_t*)p_data;
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FILTER_PHASE_BASE, filter, TRANS_NUM);
}

static void write_filter(const volatile uint8_t* p_data) {
  uint8_t flag = p_data[1];
  if (flag == FILTER_ADD_DUTY) {
    write_duty_filter(p_data + 2);
  } else if (flag == FILTER_ADD_PHASE) {
    write_phase_filter(p_data + 2);
  }
}

static void write_gain(const volatile uint8_t* p_data) {
  uint8_t flag = p_data[1];
  const uint16_t* src = (const uint16_t*)(&p_data[2]);
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr = get_addr(BRAM_SELECT_NORMAL, 0);
  uint32_t cnt = TRANS_NUM;
  volatile uint16_t* dst = &base[addr];

  _fpga_flags_internal &= ~CTL_FLAG_OP_MODE;
  if ((flag & GAIN_FLAG_LEGACY) == GAIN_FLAG_LEGACY) {
    while (cnt--) {
      *dst = *src++;
      dst += 2;
    }
    _fpga_flags_internal |= CTL_FLAG_LEGACY_MODE;
  } else if ((flag & GAIN_FLAG_DUTY) == GAIN_FLAG_DUTY) {
    dst++;
    while (cnt--) {
      *dst = *src++;
      dst += 2;
    }
    _fpga_flags_internal &= ~CTL_FLAG_LEGACY_MODE;
  } else {
    while (cnt--) {
      *dst = *src++;
      dst += 2;
    }
    _fpga_flags_internal &= ~CTL_FLAG_LEGACY_MODE;
  }
}

inline static void change_stm_segment(uint16_t segment) {
  asm("dmb");
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_MEM_SEGMENT, segment);
  asm("dmb");
}

static void write_focus_stm(const volatile uint8_t* p_data) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint8_t flag = p_data[1];
  uint16_t size;

  uint16_t addr;
  volatile uint16_t* dst;
  const uint16_t* src;
  uint32_t freq_div;
  uint32_t sound_speed;
  uint16_t start_idx;
  uint16_t finish_idx;
  uint32_t cnt;
  uint32_t segment_capacity;

  size = *((const uint16_t*)&p_data[2]);
  if ((flag & FOCUS_STM_FLAG_BEGIN) == FOCUS_STM_FLAG_BEGIN) {
    _stm_cycle = 0;
    change_stm_segment(0);

    freq_div = *((const uint32_t*)&p_data[4]);
    sound_speed = *((const uint32_t*)&p_data[8]);
    start_idx = *((const uint16_t*)&p_data[12]);
    finish_idx = *((const uint16_t*)&p_data[14]);

    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SOUND_SPEED_0, (uint16_t*)&sound_speed, sizeof(uint32_t) >> 1);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FINISH_IDX, finish_idx);

    if ((flag & FOCUS_STM_FLAG_USE_START_IDX) == FOCUS_STM_FLAG_USE_START_IDX) {
      _fpga_flags_internal |= CTL_FLAG_USE_STM_START_IDX;
    } else {
      _fpga_flags_internal &= ~CTL_FLAG_USE_STM_START_IDX;
    }
    if ((flag & FOCUS_STM_FLAG_USE_FINISH_IDX) == FOCUS_STM_FLAG_USE_FINISH_IDX) {
      _fpga_flags_internal |= CTL_FLAG_USE_STM_FINISH_IDX;
    } else {
      _fpga_flags_internal &= ~CTL_FLAG_USE_STM_FINISH_IDX;
    }

    src = (const uint16_t*)(&p_data[16]);
  } else {
    src = (const uint16_t*)(&p_data[4]);
  }

  segment_capacity = (_stm_cycle & ~FOCUS_STM_BUF_SEGMENT_SIZE_MASK) + FOCUS_STM_BUF_SEGMENT_SIZE - _stm_cycle;
  if (size <= segment_capacity) {
    cnt = size;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_cycle += size;
  } else {
    cnt = segment_capacity;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_cycle += segment_capacity;

    change_stm_segment((_stm_cycle & ~FOCUS_STM_BUF_SEGMENT_SIZE_MASK) >> FOCUS_STM_BUF_SEGMENT_SIZE_WIDTH);

    cnt = size - segment_capacity;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_cycle += size - segment_capacity;
  }

  if ((flag & FOCUS_STM_FLAG_END) == FOCUS_STM_FLAG_END) {
    _fpga_flags_internal |= CTL_FLAG_OP_MODE;
    _fpga_flags_internal &= ~CTL_FLAG_STM_GAIN_MODE;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
  }
}

static void write_gain_stm_legacy(const volatile uint8_t* p_data) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;

  uint8_t flag = p_data[1];
  uint8_t send = (flag >> 6) + 1;

  uint16_t addr;
  volatile uint16_t* dst;
  const volatile uint16_t *src, *src_base;
  uint32_t freq_div;
  uint16_t start_idx;
  uint16_t finish_idx;
  uint32_t cnt;
  uint16_t phase;

  if ((flag & GAIN_STM_FLAG_BEGIN) == GAIN_STM_FLAG_BEGIN) {
    _stm_cycle = 0;
    change_stm_segment(0);

    _gain_stm_mode = *((const uint16_t*)&p_data[2]);

    freq_div = *((const uint32_t*)&p_data[4]);
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);

    start_idx = *((const uint16_t*)&p_data[8]);
    finish_idx = *((const uint16_t*)&p_data[10]);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FINISH_IDX, finish_idx);

    if ((flag & GAIN_STM_FLAG_USE_START_IDX) == GAIN_STM_FLAG_USE_START_IDX) {
      _fpga_flags_internal |= CTL_FLAG_USE_STM_START_IDX;
    } else {
      _fpga_flags_internal &= ~CTL_FLAG_USE_STM_START_IDX;
    }
    if ((flag & GAIN_STM_FLAG_USE_FINISH_IDX) == GAIN_STM_FLAG_USE_FINISH_IDX) {
      _fpga_flags_internal |= CTL_FLAG_USE_STM_FINISH_IDX;
    } else {
      _fpga_flags_internal &= ~CTL_FLAG_USE_STM_FINISH_IDX;
    }

    src_base = (const uint16_t*)(&p_data[12]);
  } else {
    src_base = (const uint16_t*)(&p_data[2]);
  }

  src = src_base;
  addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);

  switch (_gain_stm_mode) {
    case GAIN_STM_MODE_DUTY_PHASE_FULL:
      dst = &base[addr];
      _stm_cycle += 1;
      cnt = TRANS_NUM;
      while (cnt--) *dst++ = *src++;
      break;
    case GAIN_STM_MODE_PHASE_FULL:
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) *dst++ = 0xFF00 | ((*src++) & 0x00FF);
      _stm_cycle += 1;

      if (send > 1) {
        src = src_base;
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        dst = &base[addr];
        cnt = TRANS_NUM;
        while (cnt--) *dst++ = 0xFF00 | (((*src++) >> 8) & 0x00FF);
        _stm_cycle += 1;
      }
      break;
    case GAIN_STM_MODE_PHASE_HALF:
      dst = &base[addr];
      cnt = TRANS_NUM;
      while (cnt--) {
        phase = (*src++) & 0x000F;
        *dst++ = 0xFF00 | (phase << 4) | phase;
      }
      _stm_cycle += 1;

      if (send > 1) {
        src = src_base;
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        dst = &base[addr];
        cnt = TRANS_NUM;
        while (cnt--) {
          phase = ((*src++) >> 4) & 0x000F;
          *dst++ = 0xFF00 | (phase << 4) | phase;
        }
        _stm_cycle += 1;
      }

      if (send > 2) {
        src = src_base;
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        dst = &base[addr];
        cnt = TRANS_NUM;
        while (cnt--) {
          phase = ((*src++) >> 8) & 0x000F;
          *dst++ = 0xFF00 | (phase << 4) | phase;
        }
        _stm_cycle += 1;
      }

      if (send > 3) {
        src = src_base;
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        dst = &base[addr];
        cnt = TRANS_NUM;
        while (cnt--) {
          phase = ((*src++) >> 12) & 0x000F;
          *dst++ = 0xFF00 | (phase << 4) | phase;
        }
        _stm_cycle += 1;
      }
      break;
    default:
      break;
  }

  if ((_stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) == 0)
    change_stm_segment((_stm_cycle & ~GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) >> GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH);

  if ((flag & GAIN_STM_FLAG_END) == GAIN_STM_FLAG_END) {
    _fpga_flags_internal |= CTL_FLAG_LEGACY_MODE;
    _fpga_flags_internal |= CTL_FLAG_OP_MODE;
    _fpga_flags_internal |= CTL_FLAG_STM_GAIN_MODE;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
  }
}

static void write_gain_stm_advanced(const volatile uint8_t* p_data) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;

  uint8_t flag = p_data[1];

  uint16_t addr;
  volatile uint16_t* dst;
  const volatile uint16_t *src, *src_base;
  uint32_t freq_div;
  uint16_t start_idx;
  uint16_t finish_idx;
  uint32_t cnt;

  if ((flag & GAIN_STM_FLAG_BEGIN) == GAIN_STM_FLAG_BEGIN) {
    _stm_cycle = 0;
    change_stm_segment(0);

    _gain_stm_mode = *((const uint16_t*)&p_data[2]);

    freq_div = *((const uint32_t*)&p_data[4]);
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);

    start_idx = *((const uint16_t*)&p_data[8]);
    finish_idx = *((const uint16_t*)&p_data[10]);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_START_IDX, start_idx);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FINISH_IDX, finish_idx);

    if ((flag & GAIN_STM_FLAG_USE_START_IDX) == GAIN_STM_FLAG_USE_START_IDX) {
      _fpga_flags_internal |= CTL_FLAG_USE_STM_START_IDX;
    } else {
      _fpga_flags_internal &= ~CTL_FLAG_USE_STM_START_IDX;
    }
    if ((flag & GAIN_STM_FLAG_USE_FINISH_IDX) == GAIN_STM_FLAG_USE_FINISH_IDX) {
      _fpga_flags_internal |= CTL_FLAG_USE_STM_FINISH_IDX;
    } else {
      _fpga_flags_internal &= ~CTL_FLAG_USE_STM_FINISH_IDX;
    }

    src_base = (const uint16_t*)(&p_data[12]);
  } else {
    src_base = (const uint16_t*)(&p_data[2]);
  }

  src = src_base;
  addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);

  switch (_gain_stm_mode) {
    case GAIN_STM_MODE_DUTY_PHASE_FULL:
      if ((flag & GAIN_STM_FLAG_DUTY) == GAIN_STM_FLAG_DUTY) {
        dst = &base[addr] + 1;
        _stm_cycle += 1;
      } else {
        dst = &base[addr];
      }
      cnt = TRANS_NUM;
      while (cnt--) {
        *dst = *src++;
        dst += 2;
      }
      break;
    case GAIN_STM_MODE_PHASE_FULL:
      dst = &base[addr];
      cnt = 0;
      while (cnt++ < TRANS_NUM) {
        *dst++ = *src++;
        *dst++ = _cycle[cnt] >> 1;
      }
      _stm_cycle += 1;
      break;
    case GAIN_STM_MODE_PHASE_HALF:
      // Not supported in advanced mode
      break;
    default:
      break;
  }

  if ((_stm_cycle & GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) == 0)
    change_stm_segment((_stm_cycle & ~GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) >> GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH);

  if ((flag & GAIN_STM_FLAG_END) == GAIN_STM_FLAG_END) {
    _fpga_flags_internal &= ~CTL_FLAG_LEGACY_MODE;
    _fpga_flags_internal |= CTL_FLAG_OP_MODE;
    _fpga_flags_internal |= CTL_FLAG_STM_GAIN_MODE;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
  }
}

static void write_gain_stm(const volatile uint8_t* p_data) {
  uint8_t flag = p_data[1];
  if ((flag & GAIN_STM_FLAG_LEGACY) == GAIN_STM_FLAG_LEGACY) {
    write_gain_stm_legacy(p_data);
  } else {
    write_gain_stm_advanced(p_data);
  }
}

static void clear(void) {
  uint32_t freq_div_4k = 40960;

  _read_fpga_info = false;

  _fpga_flags = 0;
  _fpga_flags_internal = 0;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG, _fpga_flags_internal | _fpga_flags);

  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, 10);

  _stm_cycle = 0;

  _mod_cycle = 2;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, (uint16_t*)&freq_div_4k, sizeof(uint32_t) >> 1);
  change_mod_segment(0);
  bram_write(BRAM_SELECT_MOD, 0, 0x0000);

  bram_set(BRAM_SELECT_NORMAL, 0, 0x0000, TRANS_NUM << 1);

  bram_set(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_DELAY_BASE, 0x0000, TRANS_NUM);
  bram_set(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FILTER_DUTY_BASE, 0x0000, TRANS_NUM);
  bram_set(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FILTER_PHASE_BASE, 0x0000, TRANS_NUM);
}

inline static uint16_t get_cpu_version(void) { return CPU_VERSION_MAJOR; }
inline static uint16_t get_cpu_version_minor(void) { return CPU_VERSION_MINOR; }
inline static uint16_t get_fpga_version(void) { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_VERSION_NUM); }
inline static uint16_t get_fpga_version_minor(void) { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_VERSION_NUM_MINOR); }
inline static uint16_t read_fpga_info(void) { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FPGA_INFO); }

void init_app(void) { clear(); }

void handle_payload(uint8_t tag, const volatile uint8_t* p_data) {
  switch (tag) {
    case TAG_NONE:
      break;
    case TAG_CLEAR:
      clear();
      break;
    case TAG_SYNC:
      synchronize(p_data + 2);
      break;
    case TAG_FIRM_INFO:
      switch (p_data[1]) {
        case INFO_TYPE_CPU_VERSION_MAJOR:
          _read_fpga_info = false;
          _rx_data = get_cpu_version() & 0xFF;
          break;
        case INFO_TYPE_CPU_VERSION_MINOR:
          _read_fpga_info = false;
          _rx_data = get_cpu_version_minor() & 0xFF;
          break;
        case INFO_TYPE_FPGA_VERSION_MAJOR:
          _read_fpga_info = false;
          _rx_data = get_fpga_version() & 0xFF;
          break;
        case INFO_TYPE_FPGA_VERSION_MINOR:
          _read_fpga_info = false;
          _rx_data = get_fpga_version_minor() & 0xFF;
          break;
        case INFO_TYPE_FPGA_FUNCTIONS:
          _read_fpga_info = false;
          _rx_data = get_fpga_version() >> 8;
          break;
        case INFO_TYPE_CLEAR:
          if (_read_fpga_info) {
            _rx_data = read_fpga_info();
          } else {
            _rx_data = 0;
          }
          break;
      }
      break;
    case TAG_UPDATE_FLAGS:
      break;
    case TAG_MODULATION:
      write_mod(p_data);
      break;
    case TAG_MODULATION_DELAY:
      write_mod_delay(p_data + 2);
      break;
    case TAG_SILENCER:
      config_silencer(p_data + 2);
      break;
    case TAG_GAIN:
      write_gain(p_data);
      break;
    case TAG_FOCUS_STM:
      write_focus_stm(p_data);
      break;
    case TAG_GAIN_STM:
      write_gain_stm(p_data);
      break;
    case TAG_FILTER:
      write_filter(p_data);
      break;
  }
}

void update(void) {
  volatile uint8_t* p_data;
  Header* header;

  if (ECATC.AL_STATUS_CODE.WORD == 0x001A) {  // Synchronization error
    if (_wdt_cnt < 0) return;
    if (_wdt_cnt-- == 0) clear();
  } else {
    _wdt_cnt = WDT_CNT_MAX;
  }

  if (_read_fpga_info) {
    _rx_data = read_fpga_info();
  }

  if (pop(&_data)) {
    p_data = (volatile uint8_t*)&_data;
    header = (Header*)p_data;
    _ack = header->msg_id;

    _read_fpga_info = (header->fpga_ctl_flag & READS_FPGA_INFO) == READS_FPGA_INFO;
    if (_read_fpga_info) _rx_data = read_fpga_info();
    _fpga_flags = header->fpga_ctl_flag;

    handle_payload(p_data[sizeof(Header)], &p_data[sizeof(Header)]);

    if (header->slot_2_offset != 0) {
      handle_payload(p_data[sizeof(Header) + header->slot_2_offset], &p_data[sizeof(Header) + header->slot_2_offset]);
    }

    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG, _fpga_flags_internal | _fpga_flags);
  }

  _sTx.ack = (((uint16_t)_ack) << 8) | _rx_data;
}

static uint8_t _last_msg_id = 0;
void recv_ethercat(uint16_t* p_data) {
  Header* header = (Header*)p_data;
  if (header->msg_id == _last_msg_id) return;
  if (push(p_data)) _last_msg_id = header->msg_id;
}
