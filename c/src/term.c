#include <errno.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <unistd.h>

#include "escape.h"

static struct termios original_term_settings;

int term_enable_raw() {
    if (tcgetattr(STDOUT_FILENO, &original_term_settings) < 0) {
        return -1;
    }

    struct termios raw_term_settings = original_term_settings;

    raw_term_settings.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    raw_term_settings.c_oflag &= ~(OPOST);
    raw_term_settings.c_cflag |= (CS8);
    raw_term_settings.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
    raw_term_settings.c_cc[VMIN] = 0;  // min bytes read() needs to return
    raw_term_settings.c_cc[VTIME] = 1; // max wait time before read() returns,
                                       // in 10ths of a second

    if (tcsetattr(STDOUT_FILENO, TCSADRAIN, &raw_term_settings) < 0) {
        return -1;
    }

    return 0;
}

int term_disable_raw() {
    if (tcsetattr(STDOUT_FILENO, TCSADRAIN, &original_term_settings) < 0) {
        return -1;
    }

    return 0;
}

int term_reset() {
    if (write(STDOUT_FILENO, ANSI_ERASE_ALL, 4) < 0)
        return -1;
    if (write(STDOUT_FILENO, ANSI_CURSOR_HOME, 3) < 0)
        return -1;

    return 0;
}

int term_get_dimensions(int* rows, int* cols) {
    struct winsize ws;

    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
        return -1;
    }
    
    *cols = ws.ws_col;
    *rows = ws.ws_row;
    return 0;
}

int term_read_key(int* need_abort, unsigned int* key) {
    int nread;
    char c;
    while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
        if (nread == -1 && errno != EAGAIN)
            return -1;

        if (*need_abort) {
            *key = '\0';
            return 0; // Stop waiting for input; we need to handle something
        }
    }

    *key = c;
    return 0;
}

int term_write(char* buf, size_t len) {
    if (write(STDOUT_FILENO, buf, len) < 0)
        return -1;

    return 0;
}
