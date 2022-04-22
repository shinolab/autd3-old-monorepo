/*
 * File: app.c
 * Project: src
 * Created Date: 22/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/04/2022
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

extern RX_STR0 _sRx0;
extern RX_STR1 _sRx1;
extern TX_STR _sTx;

// fire when ethercat packet arrives
extern void recv_ethercat(void);
// fire once after power on
extern void init_app(void);
// fire periodically with 1ms interval
extern void update(void);

typedef struct {
  uint8_t msg_id;
  uint8_t fpga_ctrl_flags;
  uint8_t cpu_ctrl_flags;
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

uint16_t _ctl_reg;

static volatile uint32_t _mod_cycle = 0;
static volatile bool_t _mod_buf_write_end = 0;

void synchronize(uint16_t* p_cycle, uint16_t ecat_sync_cycle_ticks) {
  int i;
  uint16_t t;
  volatile uint64_t next_sync0 = ECATC.DC_CYC_START_TIME.LONGLONG;

  bram_writes(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CYCLE_BASE, p_cycle, TRANS_NUM);
  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_EC_SYNC_CYCLE_TICKS, ecat_sync_cycle_ticks);
  bram_writes_volatile(BRAM_SELECT_CONTROLLER, BRAM_ADDR_EC_SYNC_TIME_0, &next_sync0, sizeof(uint64_t) >> 1);

  bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, _ctl_reg | (1 << CTL_REG_SYNC_BIT));
}

void rst_wdt() {
  _ctl_reg ^= (1 << CTL_REG_WDT_RST_BIT);
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
    bram_writes(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, &freq_div, sizeof(uint32_t) >> 1);
    data = (uint16_t*)header->MOD_HEAD.data;
    write = header->MOD_HEAD.size;
  } else {
    data = (uint16_t*)header->MOD_BODY.data;
    write = header->MOD_BODY.size;
  }

  segment_capacity = (_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) + MOD_BUF_SEGMENT_SIZE - _mod_cycle;
  if (write <= segment_capacity) {
    bram_writes(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, (write + 1) >> 1);
    _mod_cycle += mod_write;
  } else {
    bram_writes(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, segment_capacity >> 1);
    _mod_cycle += segment_capacity;
    data += segment_capacity;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET, (_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) >> MOD_BUF_SEGMENT_SIZE_WIDTH);
    bram_writes(BRAM_SELECT_MOD, (_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data, (write - segment_capacity + 1) >> 1);
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
  bram_writes(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, &step, sizeof(uint16_t) >> 1);
  bram_writes(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, &cycle, sizeof(uint32_t) >> 1);
}

static void write_normal_op() {
  // TODO: how to write 13bit/13bit?
}

void init_app(void) {}

void update(void) {}

void recv_ethercat(void) { rst_wdt(); }

/////////////////////////////////////////////////////////////////

// #define MICRO_SECONDS (1000)

// #define SEQ_BUF_FOCI_SEGMENT_SIZE (0xFFF)
// #define SEQ_BUF_GAIN_SEGMENT_SIZE (0x3F)
// #define MOD_BUF_SEGMENT_SIZE (0x7FFF)

// #define BRAM_CONFIG_SELECT (0)
// #define BRAM_MOD_SELECT (1)
// #define BRAM_TR_SELECT (2)
// #define BRAM_SEQ_SELECT (3)

// #define CONFIG_CTRL_FLAG (0x00)
// #define CONFIG_FPGA_INFO (0x01)
// #define CONFIG_SEQ_CYCLE (0x02)
// #define CONFIG_SEQ_DIV (0x03)
// #define CONFIG_MOD_BRAM_OFFSET (0x06)
// #define CONFIG_SEQ_BRAM_OFFSET (0x07)
// #define CONFIG_WAVELENGTH_UM (0x08)
// #define CONFIG_SEQ_SYNC_TIME_BASE (0x09)
// #define CONFIG_MOD_CYCLE (0x0D)
// #define CONFIG_MOD_DIV (0x0E)
// #define CONFIG_MOD_SYNC_TIME_BASE (0x0F)
// #define CONFIG_CLK_INIT_FLAG (0x13)
// #define CONFIG_SILENT_STEP (0x14)
// #define CONFIG_FPGA_VER (0x3F)

// #define TR_DELAY_OFFSET_BASE_ADDR (0x100)

// #define CP_MOD_INIT (0x0001)
// #define CP_SEQ_INIT (0x0002)

// #define OP_MODE_NORMAL (0)
// #define SEQ_MODE_POINT (0)

// #define MSG_CLEAR (0x00)
// #define MSG_RD_CPU_V_LSB (0x01)
// #define MSG_RD_CPU_V_MSB (0x02)
// #define MSG_RD_FPGA_V_LSB (0x03)
// #define MSG_RD_FPGA_V_MSB (0x04)

// #define GAIN_DATA_MODE_PHASE_DUTY_FULL (0x0001)
// #define GAIN_DATA_MODE_PHASE_FULL (0x0002)
// #define GAIN_DATA_MODE_PHASE_HALF (0x0004)

// static volatile uint8_t _msg_id = 0;
// static volatile uint16_t _ctrl_flag = 0;
// static volatile bool_t _read_fpga_info = false;

// static volatile uint32_t _seq_cycle = 0;
// static volatile uint32_t _seq_buf_fpga_write = 0;
// static volatile bool_t _seq_buf_write_end = 0;
// static volatile uint16_t _seq_gain_data_mode = GAIN_DATA_MODE_PHASE_DUTY_FULL;
// static volatile uint16_t _seq_gain_size = 0;

// static volatile bool_t _wait_on_sync = false;
// static volatile uint16_t _delay_rst = 0;

// static volatile uint16_t _ack = 0;

// typedef enum {
//   OUTPUT_ENABLE = 1 << 0,
//   OUTPUT_BALANCE = 1 << 1,
//   READS_FPGA_INFO = 1 << 2,
//   SILENT = 1 << 3,
//   FORCE_FAN = 1 << 4,
//   OP_MODE = 1 << 5,
//   SEQ_MODE = 1 << 6,
//   //
// } FPGAControlFlags;

// typedef enum {
//   MOD_BEGIN = 1 << 0,
//   MOD_END = 1 << 1,
//   SEQ_BEGIN = 1 << 2,
//   SEQ_END = 1 << 3,
//   SET_SILENT_STEP = 1 << 4,
//   DELAY_OFFSET = 1 << 5,
//   WRITE_BODY = 1 << 6,
//   WAIT_ON_SYNC = 1 << 7,
// } CPUControlFlags;

// inline static uint16_t get_cpu_version(void) { return CPU_VERSION; }
// inline static uint16_t get_fpga_version(void) { return bram_read(BRAM_CONFIG_SELECT, CONFIG_FPGA_VER); }
// inline static uint16_t read_fpga_info(void) { return bram_read(BRAM_CONFIG_SELECT, CONFIG_FPGA_INFO); }

// static void clear(void) {
//   volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
//   uint16_t addr;

//   _ctrl_flag = SILENT;
//   _read_fpga_info = false;
//   bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctrl_flag);
//   bram_write(BRAM_CONFIG_SELECT, CONFIG_SILENT_STEP, 1);

//   _seq_cycle = 0;
//   _seq_buf_fpga_write = 0;
//   _seq_buf_write_end = false;

//   _mod_cycle = 4000;
//   _mod_buf_fpga_write = 0;
//   _mod_buf_write_end = false;
//   bram_write(BRAM_CONFIG_SELECT, CONFIG_MOD_CYCLE, max(1, _mod_cycle) - 1);
//   bram_write(BRAM_CONFIG_SELECT, CONFIG_MOD_DIV, 10 - 1);
//   addr = get_addr(BRAM_MOD_SELECT, 0);
//   word_set_volatile(&base[addr], 0xFFFF, _mod_cycle >> 1);

//   addr = get_addr(BRAM_TR_SELECT, 0);
//   word_set_volatile(&base[addr], 0x0000, TRANS_NUM);

//   addr = get_addr(BRAM_TR_SELECT, TR_DELAY_OFFSET_BASE_ADDR);
//   word_set_volatile(&base[addr], 0xFF00, TRANS_NUM);
//   _delay_rst = 0;
//   bram_write(BRAM_TR_SELECT, TR_DELAY_OFFSET_BASE_ADDR + TRANS_NUM, _delay_rst);
// }

// static void recv_point_seq(void) {
//   GlobalHeader *header = (GlobalHeader *)(_sRx1.data);
//   volatile uint16_t *base = (uint16_t *)FPGA_BASE;
//   uint16_t seq_div;
//   uint16_t wavelength;
//   uint16_t seq_size = _sRx0.data[0];
//   uint32_t offset = 1;
//   volatile Focus *foci;
//   uint16_t i, addr;

//   if ((header->cpu_ctrl_flags & SEQ_BEGIN) != 0) {
//     _seq_cycle = 0;
//     _seq_buf_fpga_write = 0;
//     _seq_buf_write_end = false;
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_BRAM_OFFSET, 0);
//     seq_div = _sRx0.data[1];
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_DIV, seq_div);
//     wavelength = _sRx0.data[2];
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_WAVELENGTH_UM, wavelength);
//     offset += 4;
//   }

//   foci = (volatile Focus *)&_sRx0.data[offset];
//   for (i = 0; i < seq_size; i++) {
//     // 2bit left shift = *4=sizeof(Focus)/16
//     addr = get_addr(BRAM_SEQ_SELECT, (_seq_buf_fpga_write & SEQ_BUF_FOCI_SEGMENT_SIZE) << 2);
//     word_cpy_volatile(&base[addr], (volatile uint16_t *)&foci[i], sizeof(Focus) >> 1);
//     _seq_buf_fpga_write++;
//     // 12bit right shift = /SEQ_BUF_FOCI_SEGMENT_SIZE
//     if ((_seq_buf_fpga_write & SEQ_BUF_FOCI_SEGMENT_SIZE) == 0) bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_BRAM_OFFSET, _seq_buf_fpga_write >> 12);
//   }
//   _seq_cycle += seq_size;

//   if ((header->cpu_ctrl_flags & SEQ_END) != 0) {
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_CYCLE, max(1, _seq_cycle) - 1);
//     _seq_buf_write_end = true;
//   }
// }

// static void recv_gain_seq(void) {
//   GlobalHeader *header = (GlobalHeader *)(_sRx1.data);
//   volatile uint16_t *base = (uint16_t *)FPGA_BASE;
//   uint16_t seq_div;
//   uint16_t addr;
//   uint8_t i;
//   uint16_t duty = 0xFF00;
//   uint16_t phase;

//   if ((header->cpu_ctrl_flags & SEQ_BEGIN) != 0) {
//     _seq_cycle = 0;
//     _seq_buf_fpga_write = 0;
//     _seq_buf_write_end = false;
//     _seq_gain_data_mode = _sRx0.data[0];
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_BRAM_OFFSET, 0);
//     seq_div = _sRx0.data[1];
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_DIV, seq_div);
//     _seq_gain_size = _sRx0.data[2];
//     return;
//   }

//   // sizeof(SeqFocus) is 64 bit, thus, the memory address of the Gain data in Sequence is aligned to 64
//   // the number of transducers is 249, so the size of a Gain in Sequence is 16*256
//   addr = get_addr(BRAM_SEQ_SELECT, (_seq_cycle & SEQ_BUF_GAIN_SEGMENT_SIZE) << 8);

//   switch (_seq_gain_data_mode) {
//     case GAIN_DATA_MODE_PHASE_DUTY_FULL:
//       word_cpy_volatile(&base[addr], _sRx0.data, TRANS_NUM);
//       _seq_cycle++;
//       break;
//     case GAIN_DATA_MODE_PHASE_FULL:
//       for (i = 0; i < TRANS_NUM; i++) base[addr + i] = duty | (_sRx0.data[i] & 0x00FF);
//       _seq_cycle++;
//       addr = get_addr(BRAM_SEQ_SELECT, (_seq_cycle & SEQ_BUF_GAIN_SEGMENT_SIZE) << 8);
//       for (i = 0; i < TRANS_NUM; i++) base[addr + i] = duty | ((_sRx0.data[i] >> 8) & 0x00FF);
//       _seq_cycle++;
//       break;
//     case GAIN_DATA_MODE_PHASE_HALF:
//       for (i = 0; i < TRANS_NUM; i++) {
//         phase = _sRx0.data[i] & 0x000F;
//         phase = (phase << 4) | phase;
//         base[addr + i] = duty | phase;
//       }
//       _seq_cycle++;
//       addr = get_addr(BRAM_SEQ_SELECT, (_seq_cycle & SEQ_BUF_GAIN_SEGMENT_SIZE) << 8);
//       for (i = 0; i < TRANS_NUM; i++) {
//         phase = (_sRx0.data[i] >> 4) & 0x000F;
//         phase = (phase << 4) | phase;
//         base[addr + i] = duty | phase;
//       }
//       _seq_cycle++;
//       addr = get_addr(BRAM_SEQ_SELECT, (_seq_cycle & SEQ_BUF_GAIN_SEGMENT_SIZE) << 8);
//       for (i = 0; i < TRANS_NUM; i++) {
//         phase = (_sRx0.data[i] >> 8) & 0x000F;
//         phase = (phase << 4) | phase;
//         base[addr + i] = duty | phase;
//       }
//       _seq_cycle++;
//       addr = get_addr(BRAM_SEQ_SELECT, (_seq_cycle & SEQ_BUF_GAIN_SEGMENT_SIZE) << 8);
//       for (i = 0; i < TRANS_NUM; i++) {
//         phase = (_sRx0.data[i] >> 12) & 0x000F;
//         phase = (phase << 4) | phase;
//         base[addr + i] = duty | phase;
//       }
//       _seq_cycle++;
//       break;
//     default:
//       word_cpy_volatile(&base[addr], _sRx0.data, TRANS_NUM);
//       _seq_cycle++;
//       break;
//   }

//   // 6bit right shift = /SEQ_BUF_GAIN_SEGMENT_SIZE
//   if ((_seq_cycle & SEQ_BUF_GAIN_SEGMENT_SIZE) == 0) bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_BRAM_OFFSET, _seq_cycle >> 6);

//   if ((header->cpu_ctrl_flags & SEQ_END) != 0) {
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_SEQ_CYCLE, max(1, _seq_gain_size) - 1);
//     _seq_buf_write_end = true;
//   }
// }

// inline static uint64_t get_next_sync0(void) {
//   volatile uint64_t next_sync0 = ECATC.DC_CYC_START_TIME.LONGLONG;
//   volatile uint64_t sys_time = ECATC.DC_SYS_TIME.LONGLONG;
//   while (next_sync0 < sys_time + 250 * MICRO_SECONDS) {
//     sys_time = ECATC.DC_SYS_TIME.LONGLONG;
//     if (sys_time > next_sync0) next_sync0 = ECATC.DC_CYC_START_TIME.LONGLONG;
//   }
//   return next_sync0;
// }

// void init_app(void) { clear(); }

// void update(void) {
//   volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
//   uint16_t addr;
//   uint64_t next_sync0;
//   if (_mod_buf_write_end && _seq_buf_write_end) {
//     _mod_buf_write_end = false;
//     _seq_buf_write_end = false;
//     next_sync0 = get_next_sync0();
//     addr = get_addr(BRAM_CONFIG_SELECT, CONFIG_MOD_SYNC_TIME_BASE);
//     word_cpy_volatile(&base[addr], (volatile uint16_t *)&next_sync0, sizeof(uint64_t) >> 1);
//     addr = get_addr(BRAM_CONFIG_SELECT, CONFIG_SEQ_SYNC_TIME_BASE);
//     word_cpy_volatile(&base[addr], (volatile uint16_t *)&next_sync0, sizeof(uint64_t) >> 1);
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_CLK_INIT_FLAG, CP_MOD_INIT | CP_SEQ_INIT);
//     if (_wait_on_sync) bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctrl_flag);
//   } else if (_mod_buf_write_end) {
//     _mod_buf_write_end = false;
//     next_sync0 = get_next_sync0();
//     addr = get_addr(BRAM_CONFIG_SELECT, CONFIG_MOD_SYNC_TIME_BASE);
//     word_cpy_volatile(&base[addr], (volatile uint16_t *)&next_sync0, sizeof(uint64_t) >> 1);
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_CLK_INIT_FLAG, CP_MOD_INIT);
//   } else if (_seq_buf_write_end) {
//     _seq_buf_write_end = false;
//     next_sync0 = get_next_sync0();
//     addr = get_addr(BRAM_CONFIG_SELECT, CONFIG_SEQ_SYNC_TIME_BASE);
//     word_cpy_volatile(&base[addr], (volatile uint16_t *)&next_sync0, sizeof(uint64_t) >> 1);
//     bram_write(BRAM_CONFIG_SELECT, CONFIG_CLK_INIT_FLAG, CP_SEQ_INIT);
//     if (_wait_on_sync) bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctrl_flag);
//   }

//   switch (_msg_id) {
//     case MSG_RD_CPU_V_LSB:
//     case MSG_RD_CPU_V_MSB:
//     case MSG_RD_FPGA_V_LSB:
//     case MSG_RD_FPGA_V_MSB:
//       break;
//     default:
//       if (_read_fpga_info) _ack = (_ack & 0xFF00) | read_fpga_info();
//       break;
//   }

//   _sTx.ack = _ack;
// }

// void recv_ethercat(void) {
//   GlobalHeader *header = (GlobalHeader *)(_sRx1.data);
//   if (header->msg_id == _msg_id) return;
//   _msg_id = header->msg_id;
//   _ack = ((uint16_t)(header->msg_id)) << 8;
//   _read_fpga_info = (header->fpga_ctrl_flags & READS_FPGA_INFO) != 0;
//   if (_read_fpga_info) _ack = (_ack & 0xFF00) | read_fpga_info();

//   switch (_msg_id) {
//     case MSG_CLEAR:
//       clear();
//       break;
//     case MSG_RD_CPU_V_LSB:
//       _ack = (_ack & 0xFF00) | (get_cpu_version() & 0xFF);
//       break;
//     case MSG_RD_CPU_V_MSB:
//       _ack = (_ack & 0xFF00) | ((get_cpu_version() >> 8) & 0xFF);
//       break;
//     case MSG_RD_FPGA_V_LSB:
//       _ack = (_ack & 0xFF00) | (get_fpga_version() & 0xFF);
//       break;
//     case MSG_RD_FPGA_V_MSB:
//       _ack = (_ack & 0xFF00) | ((get_fpga_version() >> 8) & 0xFF);
//       break;
//     default:
//       if ((header->cpu_ctrl_flags & SET_SILENT_STEP) != 0) {
//         bram_write(BRAM_CONFIG_SELECT, CONFIG_SILENT_STEP, (uint16_t)header->mod_size);
//         break;
//       }
//       _ctrl_flag = header->fpga_ctrl_flags;
//       _wait_on_sync = (header->cpu_ctrl_flags & WAIT_ON_SYNC) != 0;
//       write_mod();
//       if ((header->cpu_ctrl_flags & WRITE_BODY) == 0) {
//         bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctrl_flag);
//         break;
//       }
//       if ((header->cpu_ctrl_flags & DELAY_OFFSET) != 0) {
//         bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctrl_flag);
//         set_delay_offset();
//       } else if ((_ctrl_flag & OP_MODE) == OP_MODE_NORMAL) {
//         bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctrl_flag);
//         normal_op();
//       } else {
//         if (!_wait_on_sync) bram_write(BRAM_CONFIG_SELECT, CONFIG_CTRL_FLAG, _ctrl_flag);
//         if ((_ctrl_flag & SEQ_MODE) == SEQ_MODE_POINT)
//           recv_point_seq();
//         else
//           recv_gain_seq();
//       }
//       break;
//   }
//   _sTx.ack = _ack;
// }
