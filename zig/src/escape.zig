//! ANSI escape + control codes.

pub const erase_all = "\x1b[2J";
pub const erase_line = "\x1b[K";
pub const cursor_home = "\x1b[H";

pub const private = struct {
    pub const show_cursor = "\x1b[?25h";
    pub const hide_cursor = "\x1b[?25l";
    pub const save_screen = "\x1b[?47h";
    pub const restore_screen = "\x1b[?47l";
    pub const save_cursor = "\x1b7";
    pub const restore_cursor = "\x1b8";
};
