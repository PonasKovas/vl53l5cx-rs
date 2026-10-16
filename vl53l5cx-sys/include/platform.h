/*
 * Platform layer for vl53l5cx-sys. ST's ULD (vendor/) includes this header.
 * All bus/delay work is done through callbacks supplied by the Rust side.
 * Anything not defined here comes from -D flags set in build.rs.
 */
#ifndef _PLATFORM_H_
#define _PLATFORM_H_
#pragma once

#include <stdint.h>
#ifndef VL53L5CX_RS_BINDGEN
/* ST's plugin sources rely on platform.h pulling this in. */
#include <string.h>
#endif

/* All callbacks return 0 on success, non-zero on failure.
 * addr is the 7-bit I2C address. reg is the 16-bit register, sent big-endian. */
typedef uint8_t (*VL53L5CX_read_Func)(void *ctx, uint8_t addr, uint16_t reg,
                                      uint8_t *buf, uint32_t len);
typedef uint8_t (*VL53L5CX_write_Func)(void *ctx, uint8_t addr, uint16_t reg,
                                       const uint8_t *buf, uint32_t len);
typedef uint8_t (*VL53L5CX_wait_Func)(void *ctx, uint32_t ms);

typedef struct
{
    /* 8-bit form of the I2C address (default 0x52), as the ULD expects. */
    uint16_t address;
    /* Opaque pointer handed back to the callbacks. */
    void *ctx;
    VL53L5CX_read_Func read;
    VL53L5CX_write_Func write;
    VL53L5CX_wait_Func wait_ms;
} VL53L5CX_Platform;

#ifndef VL53L5CX_NB_TARGET_PER_ZONE
#define VL53L5CX_NB_TARGET_PER_ZONE (1U)
#endif

#define PROCESSOR_LITTLE_ENDIAN
#define SWAP_UINT16(x) (x)
#define SWAP_UINT32(x) (x)

uint8_t RdByte(VL53L5CX_Platform *p_platform, uint16_t RegisterAdress, uint8_t *p_value);
uint8_t WrByte(VL53L5CX_Platform *p_platform, uint16_t RegisterAdress, uint8_t value);
uint8_t RdMulti(VL53L5CX_Platform *p_platform, uint16_t RegisterAdress, uint8_t *p_values, uint32_t size);
uint8_t WrMulti(VL53L5CX_Platform *p_platform, uint16_t RegisterAdress, uint8_t *p_values, uint32_t size);
void SwapBuffer(uint8_t *buffer, uint16_t size);
uint8_t WaitMs(VL53L5CX_Platform *p_platform, uint32_t TimeMs);

#endif /* _PLATFORM_H_ */
