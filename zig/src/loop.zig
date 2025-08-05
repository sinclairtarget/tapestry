const std = @import("std");

pub fn run(allow_private: bool) !void {
    std.log.debug("Started main loop. Allow private: {any}", .{allow_private});
}
