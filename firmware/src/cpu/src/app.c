/*
 * File: app.c
 * Project: src
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

#include "app.h"

#include "iodefine.h"
#include "kernel.h"
#include "params.h"
#include "utils.h"

#define CPU_VERSION_MAJOR (0x8C) /* v4.1 */
#define CPU_VERSION_MINOR (0x00)

#define MOD_BUF_PAGE_SIZE_WIDTH (15)
#define MOD_BUF_PAGE_SIZE (1 << MOD_BUF_PAGE_SIZE_WIDTH)
#define MOD_BUF_PAGE_SIZE_MASK (MOD_BUF_PAGE_SIZE - 1)

#define FOCUS_STM_BUF_PAGE_SIZE_WIDTH (11)
#define FOCUS_STM_BUF_PAGE_SIZE (1 << FOCUS_STM_BUF_PAGE_SIZE_WIDTH)
#define FOCUS_STM_BUF_PAGE_SIZE_MASK (FOCUS_STM_BUF_PAGE_SIZE - 1)

#define GAIN_STM_BUF_PAGE_SIZE_WIDTH (6)
#define GAIN_STM_BUF_PAGE_SIZE (1 << GAIN_STM_BUF_PAGE_SIZE_WIDTH)
#define GAIN_STM_BUF_PAGE_SIZE_MASK (GAIN_STM_BUF_PAGE_SIZE - 1)

#define WDT_CNT_MAX (1000)

extern TX_STR _sTx;

#define BUF_SIZE (64)
static volatile RX_STR _buf[BUF_SIZE];
volatile uint8_t _write_cursor;
volatile uint8_t _read_cursor;

static volatile RX_STR _data;

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

typedef struct {
  uint8_t msg_id;
  uint8_t _fpga_ctl_flag;  // only used before v4.1.0
  uint16_t slot_2_offset;
} Header;

static volatile uint8_t _ack = 0;
static volatile uint8_t _rx_data = 0;
static volatile bool_t _read_fpga_info;
static volatile bool_t _read_fpga_info_store;

static volatile uint32_t _mod_cycle = 0;

static volatile uint32_t _stm_cycle = 0;
static volatile uint16_t _gain_stm_mode = GAIN_STM_MODE_INTENSITY_PHASE_FULL;

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

void synchronize() {
  volatile uint64_t next_sync0;
  volatile uint16_t flag;

  next_sync0 = get_next_sync0();
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_EC_SYNC_TIME_0, (volatile uint16_t*)&next_sync0, sizeof(uint64_t) >> 1);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG, _fpga_flags_internal | CTL_FLAG_SYNC);

  while (true) {
    flag = bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG);
    if ((flag & CTL_FLAG_SYNC) == 0) break;
  }
}

inline static void change_mod_page(uint16_t page) {
  asm("dmb");
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_MEM_PAGE, page);
  asm("dmb");
}

void write_mod(const volatile uint8_t* p_data) {
  uint32_t page_capacity;
  uint32_t freq_div;

  uint8_t flag = p_data[1];

  uint16_t write = (((uint16_t)p_data[3]) << 8) | (uint16_t)p_data[2];

  const uint16_t* data;
  if ((flag & MODULATION_FLAG_BEGIN) == MODULATION_FLAG_BEGIN) {
    _mod_cycle = 0;
    change_mod_page(0);
    freq_div = *((const uint32_t*)&p_data[4]);
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, (uint16_t*)&freq_div, sizeof(uint32_t) >> 1);
    data = (const uint16_t*)(&p_data[8]);
  } else {
    data = (const uint16_t*)(&p_data[4]);
  }

  page_capacity = (_mod_cycle & ~MOD_BUF_PAGE_SIZE_MASK) + MOD_BUF_PAGE_SIZE - _mod_cycle;
  if (write <= page_capacity) {
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_PAGE_SIZE_MASK) >> 1, data, (write + 1) >> 1);
    _mod_cycle += write;
  } else {
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_PAGE_SIZE_MASK) >> 1, data, page_capacity >> 1);
    _mod_cycle += page_capacity;
    data += page_capacity;
    change_mod_page((_mod_cycle & ~MOD_BUF_PAGE_SIZE_MASK) >> MOD_BUF_PAGE_SIZE_WIDTH);
    bram_cpy(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_PAGE_SIZE_MASK) >> 1, data, (write - page_capacity + 1) >> 1);
    _mod_cycle += write - page_capacity;
  }

  if ((flag & MODULATION_FLAG_END) == MODULATION_FLAG_END) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
  }
}

void config_silencer(const volatile uint8_t* p_data) {
  const uint16_t* p = (const uint16_t*)&p_data[0];
  uint16_t step_intensity = p[0];
  uint16_t step_phase = p[1];
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_INTENSITY_STEP, step_intensity);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_PHASE_STEP, step_phase);
}

static void write_mod_delay(const volatile uint8_t* p_data) {
  const uint16_t* delay = (const uint16_t*)p_data;
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_DELAY_BASE, delay, TRANS_NUM);
}

static void configure_debug(const volatile uint8_t* p_data) {
  uint8_t idx = p_data[0];
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_DEBUG_OUT_IDX, idx);
}

static void write_gain(const volatile uint8_t* p_data) {
  volatile const uint16_t* src = (volatile const uint16_t*)(&p_data[2]);
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr = get_addr(BRAM_SELECT_NORMAL, 0);
  uint32_t cnt = TRANS_NUM;
  volatile uint16_t* dst = &base[addr];

  _fpga_flags_internal &= ~CTL_FLAG_OP_MODE;
  word_cpy_volatile(dst, src, cnt);
}

inline static void change_stm_page(uint16_t page) {
  asm("dmb");
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_MEM_PAGE, page);
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
  uint32_t page_capacity;

  size = *((const uint16_t*)&p_data[2]);
  if ((flag & FOCUS_STM_FLAG_BEGIN) == FOCUS_STM_FLAG_BEGIN) {
    _stm_cycle = 0;
    change_stm_page(0);

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

  page_capacity = (_stm_cycle & ~FOCUS_STM_BUF_PAGE_SIZE_MASK) + FOCUS_STM_BUF_PAGE_SIZE - _stm_cycle;
  if (size <= page_capacity) {
    cnt = size;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & FOCUS_STM_BUF_PAGE_SIZE_MASK) << 3);
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
    cnt = page_capacity;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & FOCUS_STM_BUF_PAGE_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_cycle += page_capacity;

    change_stm_page((_stm_cycle & ~FOCUS_STM_BUF_PAGE_SIZE_MASK) >> FOCUS_STM_BUF_PAGE_SIZE_WIDTH);

    cnt = size - page_capacity;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & FOCUS_STM_BUF_PAGE_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_cycle += size - page_capacity;
  }

  if ((flag & FOCUS_STM_FLAG_END) == FOCUS_STM_FLAG_END) {
    _fpga_flags_internal |= CTL_FLAG_OP_MODE;
    _fpga_flags_internal &= ~CTL_FLAG_STM_GAIN_MODE;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
  }
}

static void write_gain_stm(const volatile uint8_t* p_data) {
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
    change_stm_page(0);

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
  addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_BUF_PAGE_SIZE_MASK) << 8);

  switch (_gain_stm_mode) {
    case GAIN_STM_MODE_INTENSITY_PHASE_FULL:
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
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_BUF_PAGE_SIZE_MASK) << 8);
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
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_BUF_PAGE_SIZE_MASK) << 8);
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
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_BUF_PAGE_SIZE_MASK) << 8);
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
        addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_BUF_PAGE_SIZE_MASK) << 8);
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

  if ((_stm_cycle & GAIN_STM_BUF_PAGE_SIZE_MASK) == 0) change_stm_page((_stm_cycle & ~GAIN_STM_BUF_PAGE_SIZE_MASK) >> GAIN_STM_BUF_PAGE_SIZE_WIDTH);

  if ((flag & GAIN_STM_FLAG_END) == GAIN_STM_FLAG_END) {
    _fpga_flags_internal |= CTL_FLAG_OP_MODE;
    _fpga_flags_internal |= CTL_FLAG_STM_GAIN_MODE;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
  }
}

static void configure_force_fan(const volatile uint8_t* p_data) {
  if (p_data[0] != 0) {
    _fpga_flags_internal |= CTL_FLAG_FORCE_FAN_EX;
  } else {
    _fpga_flags_internal &= ~CTL_FLAG_FORCE_FAN_EX;
  }
}

static void configure_reads_fpga_info(const volatile uint8_t* p_data) { _read_fpga_info = p_data[0] != 0; }

static void clear(void) {
  uint32_t freq_div_4k = 5120;

  _read_fpga_info = false;

  _fpga_flags_internal = 0;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG, _fpga_flags_internal);

  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_INTENSITY_STEP, 256);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_PHASE_STEP, 256);

  _stm_cycle = 0;

  _mod_cycle = 2;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, (uint16_t*)&freq_div_4k, sizeof(uint32_t) >> 1);
  change_mod_page(0);
  bram_write(BRAM_SELECT_MOD, 0, 0xFFFF);

  bram_set(BRAM_SELECT_NORMAL, 0, 0x0000, TRANS_NUM << 1);

  bram_set(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_DELAY_BASE, 0x0000, TRANS_NUM);

  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_DEBUG_OUT_IDX, 0xFF);
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
      synchronize();
      break;
    case TAG_FIRM_INFO:
      switch (p_data[1]) {
        case INFO_TYPE_CPU_VERSION_MAJOR:
          _read_fpga_info_store = _read_fpga_info;
          _read_fpga_info = false;
          _rx_data = get_cpu_version() & 0xFF;
          break;
        case INFO_TYPE_CPU_VERSION_MINOR:
          _rx_data = get_cpu_version_minor() & 0xFF;
          break;
        case INFO_TYPE_FPGA_VERSION_MAJOR:
          _rx_data = get_fpga_version() & 0xFF;
          break;
        case INFO_TYPE_FPGA_VERSION_MINOR:
          _rx_data = get_fpga_version_minor() & 0xFF;
          break;
        case INFO_TYPE_CLEAR:
          _read_fpga_info = _read_fpga_info_store;
          _rx_data = 0;
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
    case TAG_FORCE_FAN:
      configure_force_fan(p_data + 2);
      break;
    case TAG_READS_FPGA_INFO:
      configure_reads_fpga_info(p_data + 2);
      break;
    case TAG_DEBUG:
      configure_debug(p_data + 2);
      break;
  }
}

#define AL_STATUS_CODE_SYNC_ERR (0x001A)
#define AL_STATUS_CODE_SYNC_MANAGER_WATCHDOG (0x001B)

void update(void) {
  volatile uint8_t* p_data;
  Header* header;

  if ((ECATC.AL_STATUS_CODE.WORD == AL_STATUS_CODE_SYNC_ERR) || (ECATC.AL_STATUS_CODE.WORD == AL_STATUS_CODE_SYNC_MANAGER_WATCHDOG)) {
    if (_wdt_cnt < 0) return;
    if (_wdt_cnt-- == 0) clear();
  } else {
    _wdt_cnt = WDT_CNT_MAX;
  }

  if (pop(&_data)) {
    p_data = (volatile uint8_t*)&_data;
    header = (Header*)p_data;
    _ack = header->msg_id;

    handle_payload(p_data[sizeof(Header)], &p_data[sizeof(Header)]);

    if (header->slot_2_offset != 0) {
      handle_payload(p_data[sizeof(Header) + header->slot_2_offset], &p_data[sizeof(Header) + header->slot_2_offset]);
    }

    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_FLAG, _fpga_flags_internal);
  } else {
    dly_tsk(1);
  }

  if (_read_fpga_info) _rx_data = read_fpga_info();
  _sTx.ack = (((uint16_t)_ack) << 8) | _rx_data;
}

static uint8_t _last_msg_id = 0;
void recv_ethercat(uint16_t* p_data) {
  Header* header = (Header*)p_data;
  if (header->msg_id == _last_msg_id) return;
  if (push(p_data)) _last_msg_id = header->msg_id;
}
