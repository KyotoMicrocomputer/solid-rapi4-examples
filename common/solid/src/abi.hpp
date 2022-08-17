#pragma once
#include <stdlib.h>
#include "solid_mem.h"
#include "solid_timer.h"
#include "solid_smp.h"
#include "solid_loader.h"
#include "solid_intc.h"
#include "solid_vector.h"

#ifdef SOLID_TIMER_EACHCPU
static constexpr bool _SOLID_RS_SOLID_TIMER_EACHCPU = true;
#else
static constexpr bool _SOLID_RS_SOLID_TIMER_EACHCPU = false;
#endif

static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_OFFSET0 = offsetof(SOLID_TIMER_HANDLER, pNext);
static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_OFFSET1 = offsetof(SOLID_TIMER_HANDLER, pCallQ);
static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_OFFSET2 = offsetof(SOLID_TIMER_HANDLER, globalTick);
static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_OFFSET3 = offsetof(SOLID_TIMER_HANDLER, type);
static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_OFFSET4 = offsetof(SOLID_TIMER_HANDLER, time);
static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_OFFSET5 = offsetof(SOLID_TIMER_HANDLER, func);
static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_OFFSET6 = offsetof(SOLID_TIMER_HANDLER, param);
static constexpr size_t _SOLID_RS_SOLID_TIMER_HANDLER_SIZE = sizeof(SOLID_TIMER_HANDLER);

static constexpr size_t _SOLID_RS_SOLID_INTC_HANDLER_OFFSET0 = offsetof(SOLID_INTC_HANDLER, intno);
static constexpr size_t _SOLID_RS_SOLID_INTC_HANDLER_OFFSET1 = offsetof(SOLID_INTC_HANDLER, priority);
static constexpr size_t _SOLID_RS_SOLID_INTC_HANDLER_OFFSET2 = offsetof(SOLID_INTC_HANDLER, config);
static constexpr size_t _SOLID_RS_SOLID_INTC_HANDLER_OFFSET3 = offsetof(SOLID_INTC_HANDLER, func);
static constexpr size_t _SOLID_RS_SOLID_INTC_HANDLER_OFFSET4 = offsetof(SOLID_INTC_HANDLER, param);
static constexpr size_t _SOLID_RS_SOLID_INTC_HANDLER_SIZE = sizeof(SOLID_INTC_HANDLER);

static constexpr size_t _SOLID_RS_SOLID_CORE_MAX = SOLID_CORE_MAX;
