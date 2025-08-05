const std = @import("std");

const escape = @import("escape.zig");
const terminal = @import("terminal.zig");

const State = struct {
    last_input_key: u8,
};

pub fn run(allow_private: bool) !void {
    std.log.debug("Started main loop. Allow private: {any}", .{allow_private});

    try terminal.set_up(allow_private);
    defer {
        terminal.tear_down(allow_private) catch |e| {
            std.log.err("Error tearing down terminal: {any}", .{e});
        };
    }

    var state = State{
        .last_input_key = 0,
    };

    try draw(state);

    while (true) {
        var key: u8 = 0;
        const n_read = try terminal.read_key(&key);
        if (n_read == 0) {
            continue; // Nothing has changed; no need to update screen
        }

        const should_quit = update(&state, key);
        if (should_quit) {
            break;
        }

        try draw(state);
    }
}

fn draw(state: State) !void {
    try terminal.write(escape.erase_line);
    try terminal.write(escape.cursor_home);
    try terminal.write("Got a key: ");

    const buf = [1]u8{state.last_input_key};
    try terminal.write(&buf);
}

fn update(state: *State, key: u8) bool {
    state.last_input_key = key;

    if (state.last_input_key == 'q') {
        return true;
    }

    return false;
}
