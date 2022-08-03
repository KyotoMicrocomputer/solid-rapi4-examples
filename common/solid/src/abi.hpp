#pragma once
#include <stdlib.h>
#include "solid_timer.h"
#include "solid_smp.h"

#ifdef SOLID_TIMER_EACHCPU
static constexpr size_t DEFINED_SOLID_TIMER_EACHCPU = 1;
#else
static constexpr size_t DEFINED_SOLID_TIMER_EACHCPU = 0;
#endif

static constexpr size_t SOLID_TIMER_HANDLER_OFFSET0 = offsetof(SOLID_TIMER_HANDLER, pNext);
static constexpr size_t SOLID_TIMER_HANDLER_OFFSET1 = offsetof(SOLID_TIMER_HANDLER, pCallQ);
static constexpr size_t SOLID_TIMER_HANDLER_OFFSET2 = offsetof(SOLID_TIMER_HANDLER, globalTick);
static constexpr size_t SOLID_TIMER_HANDLER_OFFSET3 = offsetof(SOLID_TIMER_HANDLER, type);
static constexpr size_t SOLID_TIMER_HANDLER_OFFSET4 = offsetof(SOLID_TIMER_HANDLER, time);
static constexpr size_t SOLID_TIMER_HANDLER_OFFSET5 = offsetof(SOLID_TIMER_HANDLER, func);
static constexpr size_t SOLID_TIMER_HANDLER_OFFSET6 = offsetof(SOLID_TIMER_HANDLER, param);

