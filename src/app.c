/*
 * File: app.c
 * Project: src
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/04/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Hapis Lab. All rights reserved.
 *
 */

#include "app.h"

#include "iodefine.h"
#include "utils.h"

#define CPU_VERSION (0x0080) /* v2.0 */

#define MOD_BUF_SEGMENT_SIZE_WIDTH (15)
#define MOD_BUF_SEGMENT_SIZE (1 << MOD_BUF_SEGMENT_SIZE_WIDTH)
#define MOD_BUF_SEGMENT_SIZE_MASK (MOD_BUF_SEGMENT_SIZE - 1)

#define POINT_STM_BUF_SEGMENT_SIZE_WIDTH (11)
#define POINT_STM_BUF_SEGMENT_SIZE (1 << POINT_STM_BUF_SEGMENT_SIZE_WIDTH)
#define POINT_STM_BUF_SEGMENT_SIZE_MASK (POINT_STM_BUF_SEGMENT_SIZE - 1)

#define GAIN_STM_BUF_SEGMENT_SIZE_WIDTH (3)
#define GAIN_STM_BUF_SEGMENT_SIZE (1 << GAIN_STM_BUF_SEGMENT_SIZE_WIDTH)
#define GAIN_STM_BUF_SEGMENT_SIZE_MASK (GAIN_STM_BUF_SEGMENT_SIZE - 1)

#define MSG_CLEAR (0x00)
#define MSG_RD_CPU_V_LSB (0x01)
#define MSG_RD_CPU_V_MSB (0x02)
#define MSG_RD_FPGA_V_LSB (0x03)
#define MSG_RD_FPGA_V_MSB (0x04)

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
  SYNC = 1 << CTL_REG_SYNC_BIT,
  WDT_RST = 1 << CTL_REG_WDT_RST_BIT,
} FPGAControlFlags;

typedef enum {
  MOD_BEGIN = 1 << 0,
  MOD_END = 1 << 1,
  STM_BEGIN = 1 << 2,
  STM_END = 1 << 3,
  IS_DUTY = 1 << 4,
  CONFIG_SILENCER = 1 << 5,
  READS_FPGA_INFO = 1 << 6
} CPUControlFlags;

typedef struct {
  uint8_t msg_id;
  uint8_t fpga_ctl_reg;
  uint8_t cpu_ctl_reg;
  union DATA {
    struct {
      uint8_t size;
      uint32_t freq_div;
      uint8_t data[120];
    } MOD_HEAD;
    struct {
      uint8_t size;
      uint8_t data[124];
    } MOD_BODY;
    struct {
      uint16_t step;
      uint32_t cycle;
      uint8_t _data[119];
    } SILENT;
  }
} GlobalHeader;

typedef struct {
  union DATA {
    struct {
      uint16_t data[TRANS_NUM];
    } NORMAL;
    struct {
      uint16_t size;
      uint32_t freq_div;
      uint32_t sound_speed;
      uint16_t data[TRANS_NUM - 1 - 2 - 2];
    } POINT_STM_HEAD;
    struct {
      uint16_t size;
      uint16_t data[TRANS_NUM - 1];
    } POINT_STM_BODY;
    struct {
      uint32_t freq_div;
      uint16_t _data[TRANS_NUM - 2];
    } GAIN_STM_HEAD;
    struct {
      uint16_t data[TRANS_NUM];
    } GAIN_STM_BODY;
  }
} BODY;

static volatile uint16_t _ack = 0;
static volatile uint8_t _msg_id = 0;

static volatile bool_t _read_fpga_info;

static volatile uint16_t _ctl_reg;

static volatile uint32_t _mod_cycle = 0;
static volatile bool_t _mod_buf_write_end = 0;

static volatile uint32_t _stm_cycle = 0;
static volatile bool_t _stm_buf_write_end = 0;

void synchronize(uint16_t* p_cycle, uint16_t ecat_sync_cycle_ticks) {
  int i;
  uint16_t t;
  volatile uint64_t next_sync0 = ECATC.DC_CYC_START_TIME.LONGLONG;

  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CYCLE_BASE, p_cycle, TRANS_NUM);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_EC_SYNC_CYCLE_TICKS, ecat_sync_cycle_ticks);
  bram_cpy_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_EC_SYNC_TIME_0, &next_sync0, sizeof(uint64_t) >> 1);

  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, _ctl_reg);
}

void rst_wdt() {
  _ctl_reg ^= WDT_RST;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, _ctl_reg);
}

void write_mod(GlobalHeader* header) {
  uint32_t freq_div;
  uint16_t* data;
  uint32_t segment_capacity;
  uint32_t write;

  if ((header->cpu_ctrl_flags & MOD_BEGIN) != 0) {
    _mod_cycle = 0;
    _mod_buf_write_end = false;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET, 0);
    freq_div = header->MOD_HEAD.freq_div;
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, &freq_div, sizeof(uint32_t) >> 1);
    data = (uint16_t*)header->MOD_HEAD.data;
    write = header->MOD_HEAD.size;
  } else {
    data = (uint16_t*)header->MOD_BODY.data;
    write = header->MOD_BODY.size;
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

  if ((header->cpu_ctrl_flags & MOD_END) != 0) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
    _mod_buf_write_end = true;
  }
}

void config_silencer(GlobalHeader* header) {
  uint16_t step = header->SILENT.step;
  uint32_t cycle = header->SILENT.cycle;
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, &step, sizeof(uint16_t) >> 1);
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, &cycle, sizeof(uint32_t) >> 1);
}

static void write_normal_op_legacy(Body* body) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr = get_addr(BRAM_SELECT_NORMAL, 0);
  uint32_t cnt = TRANS_NUM;
  volatile uint16_t* dst = &base[addr];
  volatile uint16_t* src = body.NORMAL.data;
  while (cnt--) {
    *dst = *src++;
    dst += 2;
  }
}

static void write_normal_op_raw(Body* src, bool is_duty) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr = get_addr(BRAM_SELECT_NORMAL, 0);
  uint32_t cnt = TRANS_NUM;
  volatile uint16_t* dst = &base[addr] + (is_duty ? 1 : 0);
  volatile uint16_t* src = body.NORMAL.data;
  while (cnt-- > 0) {
    *dst = *src++;
    dst += 2;
  }
}

static void write_normal_op(GlobalHeader* header, Body* src) {
  if (header->fpga_ctrl_flags & LEGACY_MODE) {
    write_normal_op_legacy(src);
  } else {
    write_normal_op_raw(src, (header->cpu_ctrl_flags & IS_DUTY) != 0);
  }
}

static void write_point_stm(GlobalHeader* header, Body* src) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr;
  volatile uint16_t* dst;
  volatile uint16_t* src;
  uint32_t freq_div;
  uint32_t sound_speed;
  uint32_t size, cnt;
  uint32_t segment_capacity;

  if ((header->cpu_ctrl_flags & SEQ_BEGIN) != 0) {
    _stm_cycle = 0;
    _stm_buf_write_end = false;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
    freq_div = src.POINT_STM_HEAD.freq_div;
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, &freq_div, sizeof(uint32_t) >> 1);
    sound_speed = src.POINT_STM_HEAD.sound_speed;
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SOUND_SPEED_0, &sound_speed, sizeof(uint32_t) >> 1);
    size = src.POINT_STM_HEAD.size;
    src = src.POINT_STM_HEAD.data;
  } else {
    size = src.POINT_STM_BODY.size;
    src = src.POINT_STM_BODY.data;
  }

  segment_capacity = (_stm_cycle & ~POINT_STM_BUF_SEGMENT_SIZE_MASK) + POINT_STM_BUF_SEGMENT_SIZE - _stm_cycle;
  if (size <= segment_capacity) {
    cnt = size;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
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
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
    dst = &base[addr];
    while (cnt--) {
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      *dst++ = *src++;
      dst += 4;
    }
    _stm_cycle += segment_capacity;

    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET,
               (_stm_cycle & ~POINT_STM_BUF_SEGMENT_SIZE_MASK) >> POINT_STM_BUF_SEGMENT_SIZE_WIDTH);

    cnt = size - segment_capacity;
    addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
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

  if ((header->cpu_ctrl_flags & SEQ_END) != 0) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
    _stm_buf_write_end = true;
  }
}

static void write_gain_stm(GlobalHeader* header, Body* src, bool legacy, bool is_duty) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr;
  volatile uint16_t* dst;
  volatile uint16_t* src;
  uint32_t freq_div;
  uint32_t cnt;
  uint16_t segment;

  if ((header->cpu_ctrl_flags & SEQ_BEGIN) != 0) {
    _stm_cycle = 0;
    _stm_buf_write_end = false;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
    freq_div = src.GAIN_STM_HEAD.freq_div;
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, &freq_div, sizeof(uint32_t) >> 1);
    return;
  }

  src = src.GAIN_STM_BODY.data;

  addr = get_addr(BRAM_SELECT_STM, (_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);
  if (legacy) {
    dst = &base[addr];
  } else {
    dst = &base[addr] + (is_duty ? 1 : 0);
  }
  cnt = TRANS_NUM;
  while (cnt--) {
    *dst = *src++;
    dst += 2;
  }
  _stm_cycle += 1;

  if ((_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) == 0) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, (_stm_cycle & ~GAIN_STM_BUF_SEGMENT_SIZE_MASK) >> GAIN_STM_BUF_SEGMENT_SIZE_WIDTH);
  }

  if ((header->cpu_ctrl_flags & SEQ_END) != 0) {
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, max(1, _stm_cycle) - 1);
    _stm_buf_write_end = true;
  }
}

static void clear(void) {
  volatile uint16_t* base = (volatile uint16_t*)FPGA_BASE;
  uint16_t addr;
  uint32_t freq_div_40k = 4096;

  _ctl_reg = LEGACY_MODE;
  _read_fpga_info = false;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, _ctl_reg);

  uint16_t step = 10;
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, &step, sizeof(uint16_t) >> 1);
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, &freq_div_40k, sizeof(uint32_t) >> 1);

  _stm_cycle = 0;
  _stm_buf_write_end = false;

  _mod_cycle = 1;
  _mod_buf_write_end = false;
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, max(1, _mod_cycle) - 1);
  bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, &freq_div_40k, sizeof(uint32_t) >> 1);
  bram_write(BRAM_SELECT_MOD, 0, 0xFFFF);

  bram_set(BRAM_SELECT_NORMAL, 0, 0x0000, TRANS_NUM << 1);
}

inline static uint16_t get_cpu_version(void) { return CPU_VERSION; }
inline static uint16_t get_fpga_version(void) { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_VERSION_NUM); }
inline static uint16_t read_fpga_info(void) { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FPGA_INFO); }

void init_app(void) { clear(); }

void update(void) {
  switch (_msg_id) {
    case MSG_RD_CPU_V_LSB:
    case MSG_RD_CPU_V_MSB:
    case MSG_RD_FPGA_V_LSB:
    case MSG_RD_FPGA_V_MSB:
      break;
    default:
      if (_read_fpga_info) _ack = (_ack & 0xFF00) | read_fpga_info();
      break;
  }
  _sTx.ack = _ack;
}

void recv_ethercat(void) {
  rst_wdt();

  GlobalHeader* header = (GlobalHeader*)(_sRx1.data);
  Body* body = (Body*)(_sRx0.data);
  if (header->msg_id == _msg_id) return;
  _msg_id = header->msg_id;
  _ack = ((uint16_t)(header->msg_id)) << 8;
  _read_fpga_info = (header->cpu_ctrl_flags & READS_FPGA_INFO) != 0;
  if (_read_fpga_info) _ack = (_ack & 0xFF00) | read_fpga_info();

  switch (_msg_id) {
    case MSG_CLEAR:
      clear();
      break;
    case MSG_RD_CPU_V_LSB:
      _ack = (_ack & 0xFF00) | (get_cpu_version() & 0xFF);
      break;
    case MSG_RD_CPU_V_MSB:
      _ack = (_ack & 0xFF00) | ((get_cpu_version() >> 8) & 0xFF);
      break;
    case MSG_RD_FPGA_V_LSB:
      _ack = (_ack & 0xFF00) | (get_fpga_version() & 0xFF);
      break;
    case MSG_RD_FPGA_V_MSB:
      _ack = (_ack & 0xFF00) | ((get_fpga_version() >> 8) & 0xFF);
      break;
    default:
      if ((header->cpu_ctrl_flags & CONFIG_SILENCER) != 0) {
        config_silencer(header);
        break;
      }
      _ctl_reg = header->fpga_ctl_reg;
      write_mod(header);

      bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctl_reg);

      if ((_ctl_reg & OP_MODE) == 0) {
        write_normal_op(header, body);
      } else {
        if ((_ctl_reg & STM_GAIN_MODE) == 0)
          write_point_stm(header, body);
        else
          write_gain_stm(header, body);
      }
      break;
  }
  _sTx.ack = _ack;
}