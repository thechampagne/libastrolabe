#ifndef __ASTROLABE_H__
#define __ASTROLABE_H__

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef union {
  void* data;
  char* error;
} astrolabe_date_t;

typedef enum {
  ASTROLABE_NONE,
  ASTROLABE_OUT_OF_RANGE,
  ASTROLABE_INVALID_FORMAT,
} astrolabe_error;

typedef enum {
    DATE_UNIT_YEAR,
    DATE_UNIT_MONTH,
    DATE_UNIT_DAY,
} astrolabe_date_unit;

extern astrolabe_date_t astrolabe_date_now();

extern astrolabe_date_t astrolabe_date_from_ymd(int32_t year, uint32_t month, uint32_t day, astrolabe_error* error_code);

extern astrolabe_date_t astrolabe_date_from_timestamp(int64_t timestamp, astrolabe_error* error_code);

extern astrolabe_date_t astrolabe_date_from_days(int32_t days);

extern int32_t astrolabe_date_as_days(astrolabe_date_t* astrolabe_date);

extern int64_t astrolabe_date_timestamp(astrolabe_date_t* astrolabe_date);

extern uint32_t astrolabe_date_between(astrolabe_date_t* astrolabe_date, astrolabe_date_t* compare);

extern int32_t astrolabe_date_get(astrolabe_date_t* astrolabe_date, astrolabe_date_unit unit);

extern astrolabe_date_t astrolabe_date_set(astrolabe_date_t* astrolabe_date, int32_t value, astrolabe_date_unit unit, astrolabe_error* error_code);

extern astrolabe_date_t astrolabe_date_apply(astrolabe_date_t* astrolabe_date, int32_t amount, astrolabe_date_unit unit, astrolabe_error* error_code);

extern char* astrolabe_date_format(astrolabe_date_t* astrolabe_date, const char* format);

#ifdef __cplusplus
}
#endif

#endif // __ASTROLABE_H__