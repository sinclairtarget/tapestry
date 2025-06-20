#ifndef _ANSI_H_
#define _ANSI_H_

#define ANSI_GREEN "\x1B[32m"
#define ANSI_RED "\x1B[31m"
#define ANSI_RESET "\x1B[0m"

#define ANSI_ERASE_ALL "\x1B[2J"
#define ANSI_ERASE_LINE "\x1B[K"
#define ANSI_CURSOR_HOME "\x1B[H"

#define PRIVATE_SHOW_CURSOR "\x1B[?25h"
#define PRIVATE_HIDE_CURSOR "\x1B[?25l"
#define PRIVATE_SAVE_SCREEN "\x1B[?47h"
#define PRIVATE_RESTORE_SCREEN "\x1B[?47l"
#define PRIVATE_SAVE_CURSOR "\x1B" "7"
#define PRIVATE_RESTORE_CURSOR "\x1B" "8"

#endif
