pub const ERASE_ALL: &[u8] = b"\x1B[2J";
pub const ERASE_LINE: &[u8] = b"\x1B[K";
pub const CURSOR_HOME: &[u8] = b"\x1B[H";

pub const PRIVATE_SHOW_CURSOR: &[u8] = b"\x1B[?25h";
pub const PRIVATE_HIDE_CURSOR: &[u8] = b"\x1B[?25l";
pub const PRIVATE_SAVE_SCREEN: &[u8] = b"\x1B[?47h";
pub const PRIVATE_RESTORE_SCREEN: &[u8] = b"\x1B[?47l";
pub const PRIVATE_SAVE_CURSOR: &[u8] = b"\x1B7";
pub const PRIVATE_RESTORE_CURSOR: &[u8] = b"\x1B8";
