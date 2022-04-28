# Memory map

ここでは, CPUから見たMemory mapについて述べる.

- FPGA内部のBRAMは4つに分かれている. 書き込み時はCPU_ADDR ($\SI{16}{bit}$) の内, 上位$\SI{2}{bit}$ (BRAM_SELECT) でこれを区別する.
    - 0x0: Controller BRAM
    - 0x1: Modulator BRAM
    - 0x2: Normal BRAM
    - 0x3: STM BRAM
- Modulator/STM BRAMはそのままだと書き込みアドレスが足りないので, Controller BRAM内の特定のアドレスに書き込まれたオフセットを使用する.

## Controller

| BRAM_SELECT | BRAM_ADDR (9bit) | DATA (16 bit)         | R/W |
|-------------|------------------|---------------------- |-----|
| 0x0         | 0x000            | CTL_REG               | R/W |
|             | 0x001            | 7:0 = FPGA info       | W   |
|             | 0x010            | EC_SYNC_CYCLE_TICKS   | W   |
|             | 0x011            | EC_SYNC_TIME_0        | W   |
|             | 0x012            | EC_SYNC_TIME_1        | W   |
|             | 0x013            | EC_SYNC_TIME_2        | W   |
|             | 0x014            | EC_SYNC_TIME_3        | W   |
|             | 0x020            | 0 = MOD_ADDR_OFFSET   | W   |
|             | 0x021            | MOD_CYCLE     	     | W   |
|             | 0x022            | MOD_FREQ_DIV_0        | W   |
|             | 0x023            | MOD_FREQ_DIV_1        | W   |
|             | 0x03F            | VERSION_NUM           | R   |
|             | 0x040            | SILENT_CYCLE          | W   |
|             | 0x041            | SILENT_STEP   	     | W   |
|             | 0x050            | 4:0 = STM_ADDR_OFFSET | W   |
|             | 0x051            | STM_CYCLE             | W   |
|             | 0x052            | STM_FREQ_DIV_0        | W   |
|             | 0x053            | STM_FREQ_DIV_1        | W   |
|             | 0x054            | SOUND_SPEED_0         | W   |
|             | 0x055            | SOUND_SPEED_1         | W   |
|             | 0x100            | CYCLE\[0\]            | W   |
|             | ︙               | ︙                    | ︙  |
|             | 0x1F8            | CYCLE\[248\]          | W   |

* CTL_REG bit
    * 4: FORCE_FAN
    * 5: OP_MODE (0: Normal, 1: STM)
    * 6: SEQ_MODE (0: Point STM, 1: Gain STM)
    * 8: SYNC_SET

## Modulator

Modulator BRAMのaddressは\{MOD_ADDR_OFFSET, CPU_ADDR\[13:0\]\}の$\SI{15}{bit}$

| BRAM_SELECT | BRAM_ADDR (15bit) | DATA (16bit)              | R/W |
|-------------|-------------------|---------------------------|-----|
| 0x1         | 0x0000            | mod\[1\]/mod\[0\]         | W   |
|             | ︙                | ︙                        | ︙  |
|             | 0x7FFF            | mod\[65535\]/mod\[65534\] | W   |

## Normal

| BRAM_SELECT | BRAM_ADDR (9bit)  | DATA (16bit)        | R/W |
|-------------|-------------------|---------------------|-----|
| 0x2         | 0x000             | 12:0 = phase\[0\]   | W   |
|             | 0x001             | 12:0 = duty\[0\]    | W   |
|             | ︙                | ︙                  | ︙  |
|             | 0x1F0             | 12:0 = phase\[248\] | W   |
|             | 0x1F1             | 12:0 = duty\[248\]  | W   |

## STM 

STM BRAMはPoint STMとGain STMで共用である.

STM BRAMのaddressは\{STM_ADDR_OFFSET, CPU_ADDR\[13:0\]\}の$\SI{19}{bit}$

### Point STM (SEQ_MODE == 0)

| BRAM_SELECT | BRAM_ADDR (19bit) | DATA (16bit)                            | R/W |
|-------------|-------------------|-----------------------------------------|-----|
| 0x3         | 0x00000           | x\[0\]\[15:0\]                          | W   |
|             | 0x00001           | y\[0\]\[13:0\]/x\[0\]\[17:16\]          | W   |
|             | 0x00002           | z\[0\]\[11:0\]/y\[0\]\[17:14\]          | W   |
|             | 0x00003           | duty_shift\[0\]/z\[0\]\[17:12\]         | W   |
|             | 0x00004-0x00007   | unused                                  | W   |
|             | ︙                | ︙                                      | ︙  |
|             | 0x7FFF8           | x\[65535\]\[15:0\]                      | W   |
|             | 0x7FFF9           | y\[65535\]\[13:0\]/x\[65535\]\[17:16\]  | W   |
|             | 0x7FFFA           | z\[65535\]\[11:0\]/y\[65535\]\[17:14\]  | W   |
|             | 0x7FFFB           | duty_shift\[65535\]/z\[65535\]\[17:12\] | W   |
|             | 0x7FFFC-0x7FFFF   | unused                                  | W   |

### Gain STM (SEQ_MODE == 1)

| BRAM_SELECT | BRAM_ADDR (19bit) | DATA (16bit)                 | R/W |
|-------------|-------------------|------------------------------|-----|
| 0x3         | 0x00000           | 12:0 = phase\[0\]\[0\]       | W   |
|             | 0x00001           | 12:0 = duty\[0\]\[0\]        | W   |
|             | ︙                | ︙                           | ︙  |
|             | 0x001F0           | 12:0 = phase\[0\]\[248\]     | W   |
|             | 0x001F1           | 12:0 = duty\[0\]\[248\]      | W   |
|             | ︙                | ︙                           | ︙  |
|             | 0x7FE0E           | 12:0 = phase\[65535\]\[0\]   | W   |
|             | 0x7FE0F           | 12:0 = duty\[65535\]\[0\]    | W   |
|             | ︙                | ︙                           | ︙  |
|             | 0x7FFFE           | 12:0 = phase\[65535\]\[248\] | W   |
|             | 0x7FFFF           | 12:0 = duty\[65535\]\[248\]  | W   |
