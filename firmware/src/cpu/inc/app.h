// File: app.h
// Project: inc
// Created Date: 25/04/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
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

inline static void bram_cpy(uint8_t bram_select, uint16_t base_bram_addr, const uint16_t *values, uint32_t cnt) {
  uint16_t base_addr = get_addr(bram_select, base_bram_addr);
  volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
  volatile uint16_t *dst = &base[base_addr];
  while (cnt-- > 0) *dst++ = *values++;
}

inline static void bram_cpy_volatile(uint8_t bram_select, uint16_t base_bram_addr, const volatile uint16_t *values, uint32_t cnt) {
  uint16_t base_addr = get_addr(bram_select, base_bram_addr);
  volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
  volatile uint16_t *dst = &base[base_addr];
  while (cnt-- > 0) *dst++ = *values++;
}

inline static void bram_set(uint8_t bram_select, uint16_t base_bram_addr, uint16_t value, uint32_t cnt) {
  uint16_t base_addr = get_addr(bram_select, base_bram_addr);
  volatile uint16_t *base = (volatile uint16_t *)FPGA_BASE;
  volatile uint16_t *dst = &base[base_addr];
  while (cnt-- > 0) *dst++ = value;
}

inline static void memcpy_volatile(volatile void *restrict dst, const volatile void *restrict src, uint32_t cnt) {
  const volatile unsigned char *src_c = src;
  volatile unsigned char *dst_c = dst;
  while (cnt-- > 0) *dst_c++ = *src_c++;
}

inline static void memset_volatile(volatile void *restrict dst, const int value, uint32_t cnt) {
  const unsigned char value_c = value & 0xFF;
  volatile unsigned char *dst_c = dst;
  while (cnt-- > 0) *dst_c++ = value_c;
}

typedef struct {
  uint16_t data[313];
} RX_STR;

typedef struct {
  uint16_t reserved;
  uint16_t ack;
} TX_STR;

#endif /* APP_H_ */
