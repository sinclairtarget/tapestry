#ifndef _TERM_H_
#define _TERM_H_

#include <stddef.h>

int term_enable_raw();
int term_disable_raw();
int term_reset();
int term_get_dimensions(int* rows, int* cols);
int term_read_key(int* need_abort, unsigned int* key);
int term_write(char* buf, size_t len);

#endif
