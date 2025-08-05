const std = @import("std");
const c = @cImport(
    @cInclude("termios.h"),
);
const escape = @import("escape.zig");

var original_termios: std.posix.termios = undefined;

pub fn set_up(allow_private: bool) !void {
    std.log.debug("{s}", .{"Running terminal set up"});

    if (allow_private) {
        try write(escape.private.save_screen);
        try write(escape.private.save_cursor);
        try write(escape.private.hide_cursor);
    }

    try enable_raw_mode();
    try reset();
}

pub fn tear_down(allow_private: bool) !void {
    std.log.debug("{s}", .{"Running terminal tear down"});

    if (allow_private) {
        try write(escape.private.show_cursor);
        try write(escape.private.restore_cursor);
        try write(escape.private.restore_screen);
    } else {
        try reset();
    }

    try disable_raw_mode();
}

pub fn write(bytes: []const u8) !void {
    try std.io.getStdOut().writeAll(bytes);
}

pub fn read_key(key: *u8) !usize {
    var buf: [1]u8 = undefined;
    const n_read = try std.io.getStdOut().read(&buf);

    if (n_read > 0) {
        key.* = buf[0];
    }

    return n_read;
}

fn reset() !void {
    try write(escape.erase_all);
    try write(escape.cursor_home);
}

fn enable_raw_mode() !void {
    original_termios = try std.posix.tcgetattr(std.io.getStdOut().handle);

    var copy = original_termios;
    copy.iflag.BRKINT = false;
    copy.iflag.ICRNL = false;
    copy.iflag.INPCK = false;
    copy.iflag.ISTRIP = false;
    copy.iflag.IXON = false;
    copy.oflag.OPOST = false;
    copy.cflag.CSIZE = .CS8;
    copy.lflag.ECHO = false;
    copy.lflag.ICANON = false;
    copy.lflag.IEXTEN = false;
    copy.lflag.ISIG = false;
    copy.cc[c.VMIN] = 0; // min bytes read() needs to return
    copy.cc[c.VTIME] = 1; // max wait time for read() in 10ths of a second

    try std.posix.tcsetattr(
        std.io.getStdOut().handle,
        .DRAIN,
        copy,
    );
}

fn disable_raw_mode() !void {
    try std.posix.tcsetattr(
        std.io.getStdOut().handle,
        .DRAIN,
        original_termios,
    );
}
