// File: app.h
// Project: inc
// Created Date: 22/04/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/04/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#ifndef APP_H_
#define APP_H_

#ifndef true
#define true 1
#endif
#ifndef false
#define false 0
#endif
#ifndef uint8_t
typedef unsigned char uint8_t;
#endif
#ifndef uint16_t
typedef unsigned short uint16_t;
#endif
#ifndef uint32_t
typedef unsigned long uint32_t;
#endif
#ifndef uint64_t
typedef long long unsigned int uint64_t;
#endif
#ifndef bool_t
typedef int bool_t;
#endif

#define TRANS_NUM (249)

#define FPGA_BASE (0x44000000) /* CS1 FPGA address */

inline static void word_cpy(uint16_t *dst, uint16_t *src, uint32_t cnt) {
  while (cnt-- > 0) *dst++ = *src++;
}
inline static void word_cpy_volatile(volatile uint16_t *dst, volatile uint16_t *src, uint32_t cnt) {
  while (cnt-- > 0) *dst++ = *src++;
}
inline static void word_set_volatile(volatile uint16_t *dst, uint16_t v, uint32_t cnt) {
  while (cnt-- > 0) *dst++ = v;
}

inline static void memcpy_volatile(volatile void *dst, volatile const void *src, uint32_t cnt) {
  volatile uint8_t *dst_uc = dst;
  volatile const uint8_t *src_uc = src;
  while (cnt-- > 0) *dst_uc++ = *src_uc++;
}
inline static void memset_volatile(volatile void *s, uint8_t c, uint32_t cnt) {
  volatile uint8_t *p = s;
  while (cnt-- > 0) *p++ = c;
}

inline static uint16_t get_addr(uint8_t bram_select, uint16_t bram_addr) { return (((uint16_t)bram_select & 0x0003) << 14) | (bram_addr & 0x3FFF); }
inline static void bram_write(uint8_t bram_select, uint16_t bram_addr, uint16_t value) {
  volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
  uint16_t addr = get_addr(bram_select, bram_addr);
  base[addr] = value;
}
inline static uint16_t bram_read(uint8_t bram_select, uint16_t bram_addr) {
  volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
  uint16_t addr = get_addr(bram_select, bram_addr);
  return base[addr];
}
inline static void bram_writes(uint8_t bram_select, uint16_t base_bram_addr, uint16_t *values, uint32_t cnt) {
  uint16_t base_addr = get_addr(bram_select, base_bram_addr);
  volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
  volatile uint16_t *dst = &base[base_addr];
  while (cnt-- > 0) *dst++ = *values++;
}
inline static void bram_writes_volatile(uint8_t bram_select, uint16_t base_bram_addr, volatile uint16_t *values, uint32_t cnt) {
  uint16_t base_addr = get_addr(bram_select, base_bram_addr);
  volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
  volatile uint16_t *dst = &base[base_addr];
  while (cnt-- > 0) *dst++ = *values++;
}

typedef struct {
  uint16_t x15_0;
  uint16_t y13_0_x17_16;
  uint16_t z11_0_y17_14;
  uint16_t duty_z17_12;
} Focus;

typedef struct {
  uint16_t reserved;
  uint16_t data[TRANS_NUM]; /* Data from PC */
} RX_STR0;

typedef struct {
  uint16_t reserved;
  uint16_t data[64]; /* Header from PC */
} RX_STR1;

typedef struct {
  uint16_t reserved;
  uint16_t ack;
} TX_STR;

#endif /* APP_H_ */
