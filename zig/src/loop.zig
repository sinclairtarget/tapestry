const std = @import("std");
const Allocator = std.mem.Allocator;

const escape = @import("escape.zig");
const terminal = @import("terminal.zig");

const max_cols = 512;
const State = struct {
    last_input_key: u8,
    rows: u16,
    cols: u16,
};

pub fn run(gpa: Allocator, allow_private: bool) !void {
    std.log.debug("Started main loop. Allow private: {any}", .{allow_private});

    try terminal.set_up(allow_private);
    defer {
        terminal.tear_down(allow_private) catch |e| {
            std.log.err("Error tearing down terminal: {any}", .{e});
        };
    }

    const rows, const cols = try terminal.get_dimensions();
    var state = State{
        .last_input_key = 0,
        .rows = rows,
        .cols = cols,
    };
    std.log.debug("Found dimensions: {d} x {d}", .{ rows, cols });

    // Our strategy is to only free memory at the end of every "frame"
    var arena_impl = std.heap.ArenaAllocator.init(gpa);
    defer arena_impl.deinit();
    const arena = arena_impl.allocator();

    try draw(arena, state); // Initial draw
    _ = arena_impl.reset(.retain_capacity);

    while (true) {
        var key: u8 = 0;
        const n_read = try terminal.read_key(&key);
        if (n_read == 0) {
            continue; // Nothing has changed; no need to update screen
            // Also, no need to reset arena!! We haven't used it
        }

        const should_quit = try update(&state, key, false);
        if (should_quit) {
            break;
        }

        try draw(arena, state);
        _ = arena_impl.reset(.retain_capacity);
    }
}

fn draw(arena: Allocator, state: State) !void {
    try terminal.write(escape.cursor_home);

    const rows = state.rows;
    const cols = if (state.cols > max_cols) max_cols else state.cols;
    const midrow = rows / 2;

    const scratch: []u8 = try arena.alloc(u8, cols - 2); // Exclude border
    const line_scratch: []u8 = try arena.alloc(u8, cols + 2); // for \r\n
    for (0..rows) |row| {
        try terminal.write(escape.erase_line);

        const content = blk: {
            if (row == midrow - 2) {
                break :blk try std.fmt.bufPrint(
                    scratch,
                    "{d} x {d}",
                    .{ rows, cols },
                );
            } else if (row == midrow - 1) {
                if (state.last_input_key == 0) {
                    break :blk try std.fmt.bufPrint(scratch, "{s}", .{"-"});
                } else if (std.ascii.isControl(state.last_input_key)) {
                    break :blk try std.fmt.bufPrint(
                        scratch,
                        "{d}",
                        .{state.last_input_key},
                    );
                } else {
                    break :blk try std.fmt.bufPrint(
                        scratch,
                        "{0d} ('{0c}')",
                        .{state.last_input_key},
                    );
                }
            } else if (row == midrow + 1) {
                break :blk try std.fmt.bufPrint(
                    scratch,
                    "{s}",
                    .{"Press ^c or ^q to quit."},
                );
            } else {
                break :blk "";
            }
        };

        const line = blk: {
            if (row == 0) {
                break :blk try std.fmt.bufPrint(
                    line_scratch,
                    "+{[value]s:-^[width]}+\r\n",
                    .{
                        .value = "",
                        .width = cols - 2,
                    },
                );
            } else if (row < rows - 1) {
                break :blk try std.fmt.bufPrint(
                    line_scratch,
                    "|{[value]s: ^[width]}|\r\n",
                    .{
                        .value = content,
                        .width = cols - 2,
                    },
                );
            } else {
                break :blk try std.fmt.bufPrint(
                    line_scratch,
                    "+{[value]s:-^[width]}+",
                    .{
                        .value = "",
                        .width = cols - 2,
                    },
                );
            }
        };

        try terminal.write(line);
    }
}

fn update(state: *State, key: u8, need_dimensions: bool) !bool {
    state.last_input_key = key;

    if (state.last_input_key == ctrl('q') or state.last_input_key == ctrl('c')) {
        return true;
    }

    if (need_dimensions) {
        const rows, const cols = try terminal.get_dimensions();
        state.rows = rows;
        state.cols = cols;
    }

    return false;
}

fn ctrl(b: u8) u8 {
    return b & 0x1f;
}
