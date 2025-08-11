const std = @import("std");
const posix = std.posix;

var got_sigwinch: bool = false;

pub fn register_sigwinch_handler() !void {
    posix.sigaction(
        posix.SIG.WINCH,
        &posix.Sigaction{
            .handler = .{
                .handler = handle_sigwinch,
            },
            .mask = posix.empty_sigset,
            .flags = 0,
        },
        null,
    );
}

pub fn consume_sigwinch() bool {
    const val = got_sigwinch;
    if (val) {
        got_sigwinch = false;
    }

    return val;
}

fn handle_sigwinch(_: c_int) callconv(.C) void {
    got_sigwinch = true;
}
