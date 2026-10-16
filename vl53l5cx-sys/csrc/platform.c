#include "platform.h"

uint8_t RdByte(VL53L5CX_Platform *p, uint16_t reg, uint8_t *value)
{
    return p->read(p->ctx, (uint8_t)(p->address >> 1), reg, value, 1U);
}

uint8_t WrByte(VL53L5CX_Platform *p, uint16_t reg, uint8_t value)
{
    return p->write(p->ctx, (uint8_t)(p->address >> 1), reg, &value, 1U);
}

uint8_t RdMulti(VL53L5CX_Platform *p, uint16_t reg, uint8_t *values, uint32_t size)
{
    return p->read(p->ctx, (uint8_t)(p->address >> 1), reg, values, size);
}

uint8_t WrMulti(VL53L5CX_Platform *p, uint16_t reg, uint8_t *values, uint32_t size)
{
    return p->write(p->ctx, (uint8_t)(p->address >> 1), reg, values, size);
}

/* Reverses the byte order of every 32-bit word in the buffer. */
void SwapBuffer(uint8_t *buffer, uint16_t size)
{
    uint32_t i;
    for (i = 0; i + 3U < size; i += 4U)
    {
        uint8_t b0 = buffer[i];
        uint8_t b1 = buffer[i + 1U];
        buffer[i]      = buffer[i + 3U];
        buffer[i + 1U] = buffer[i + 2U];
        buffer[i + 2U] = b1;
        buffer[i + 3U] = b0;
    }
}

uint8_t WaitMs(VL53L5CX_Platform *p, uint32_t ms)
{
    return p->wait_ms(p->ctx, ms);
}
