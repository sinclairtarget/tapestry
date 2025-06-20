#include <ctype.h>
#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "term.h"
#include "escape.h"

#define MAX_COLS 512
#define CTRL(k) ((k) & 0x1f)


typedef struct {
    int window_rows;
    int window_cols;
    unsigned int last_input_key;
} State;


static int need_window_update = 0;
static int allow_private = 0;


void print_error(char* msg) {
    if (errno != 0) {
        perror(msg);
    } else {
        fprintf(stderr, "%s\n", msg);
    }
}

void die(char* msg) {
    print_error(msg);
    exit(1);
}

void setup() {
    if (allow_private) {
        if (term_write(PRIVATE_SAVE_SCREEN, 6) != 0) {
            die("Failed to write to terminal");
        }

        if (term_write(PRIVATE_SAVE_CURSOR, 2) != 0) {
            die("Failed to write to terminal");
        }

        if (term_write(PRIVATE_HIDE_CURSOR, 6) != 0) {
            die("Failed to write to terminal");
        }
    }

    if (term_enable_raw() != 0) {
        die("Failed to activate terminal raw mode");
    }

    if (term_reset() != 0) {
        die("Failed to reset terminal");
    }
}

// We don't want to call exit() in this function; we are already exiting
void teardown() {
    if (allow_private) {
        if (term_write(PRIVATE_SHOW_CURSOR, 6) != 0) {
            print_error("Failed to write to terminal");
            return;
        }

        if (term_write(PRIVATE_RESTORE_CURSOR, 2) != 0) {
            die("Failed to write to terminal");
        }

        if (term_write(PRIVATE_RESTORE_SCREEN, 6) != 0) {
            die("Failed to write to terminal");
        }
    } else {
        if (term_reset() != 0) {
            die("Failed to reset terminal");
        }
    }

    if (term_disable_raw() != 0) {
        print_error("Failed to deactivate terminal raw mode");
        return;
    }
}

void handle_sigwinch() {
    need_window_update = 1;
}

int register_sigwinch_handler() {
    struct sigaction sa;

    sa.sa_handler = handle_sigwinch;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    if (sigaction(SIGWINCH, &sa, NULL) == -1)
        return -1;

    return 0;
}

void draw(State* state) {
    char line[MAX_COLS + 1];
    char buf[MAX_COLS + 5];

    if (term_write(ANSI_CURSOR_HOME, 3) != 0)
        die("Failed to write to terminal");

    int rows = state->window_rows;
    int cols = state->window_cols;
    if (cols > MAX_COLS)
        cols = MAX_COLS;
    int linecols = cols - 2;

    int midrow = state->window_rows / 2;

    for (int row = 0; row < rows; row++) {
        if (row == 0 || row == rows - 1) {
            int col;
            for (col = 0; col < linecols; col++)
                line[col] = '-';
            line[col] = '\0';
        } else if (row == midrow - 2) {
            sprintf(line, "%d x %d", state->window_rows, state->window_cols);
        } else if (row == midrow - 1) {
            if (state->last_input_key == '\0') {
                strcpy(line, "-");
            } else if (iscntrl(state->last_input_key)) {
                sprintf(line, "%d", state->last_input_key);
            } else {
                sprintf(
                    line, 
                    "%d ('%c')", 
                    state->last_input_key, 
                    state->last_input_key
                );
            }
        } else if (row == midrow + 1) {
            strcpy(line, "Press ^c or ^q to quit.");
        } else {
            line[0] = '\0';
        }

        int linelen = strlen(line);
        int left_padding = (linecols - linelen) / 2;
        int right_padding = (linecols - linelen + 1) / 2;

        if (row == 0) {
            sprintf(
                buf, 
                "%s+%*s%s%*s+\r\n", 
                ANSI_ERASE_LINE, 
                left_padding, 
                "", 
                line, 
                right_padding, 
                ""
            );
        } else if (row == state->window_rows - 1) {
            sprintf(
                buf, 
                "%s+%*s%s%*s+", 
                ANSI_ERASE_LINE, 
                left_padding, 
                "", 
                line, 
                right_padding, 
                ""
            );
        } else {
            sprintf(
                buf, 
                "%s|%*s%s%*s|\r\n", 
                ANSI_ERASE_LINE, 
                left_padding, 
                "", 
                line, 
                right_padding, 
                ""
            );
        }
        if (term_write(buf, strlen(buf)) != 0) {
            die("Failed to write to terminal");
        }
    }
}

void update(State* state, unsigned int key) {
    if (need_window_update) {
        if (term_get_dimensions(&state->window_rows, &state->window_cols) != 0)
            die("Failed to get window dimensions");

        need_window_update = 0;
    }

    state->last_input_key = key;
}

void parse_args(int argc, char* argv[]) {
    if (argc > 1) {
        if (strcmp(argv[1], "--allow-private") == 0) {
            allow_private = 1;
        } else {
            printf("Usage: %s [--allow-private]\n", argv[0]);
            char buf[256];
            snprintf(
                buf, 
                255, 
                "Unrecognized command-line flag: \"%s\"", 
                argv[1]
            );
            die(buf);
        }
    }
}

int main(int argc, char* argv[]) {
    parse_args(argc, argv);

    setup();
    if (atexit(teardown) != 0) {
        die("Error registering exit handler");
    }

    State state;
    state.last_input_key = '\0';
    if (term_get_dimensions(&state.window_rows, &state.window_cols) != 0)
        die("Failed to get window dimensions");

    if (register_sigwinch_handler() != 0)
        die("Failed to register SIGWINCH handler");

    while (1) {
        draw(&state);

        unsigned int c = '\0';
        if (term_read_key(&need_window_update, &c) != 0) {
            die("Failed to read input key");
        }

        update(&state, c);

        if (c == CTRL('q') || c == CTRL('c'))
            break;
    }

    return 0;
}
