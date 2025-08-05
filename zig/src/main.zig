const std = @import("std");
const process = std.process;
const mem = std.mem;
const Allocator = mem.Allocator;
const builtin = @import("builtin");

const loop = @import("loop.zig");

pub fn main() !void {
    // This is the actual allocator.
    var gpa_impl: std.heap.GeneralPurposeAllocator(.{}) = .init;
    defer {
        const deinit_status = gpa_impl.deinit();
        if (deinit_status == .leak) {
            die("{s}\n", .{"GPA detected leaks on exit"});
        }
    }
    // This is the interface to the allocator, with type std.mem.Allocator.
    const gpa = gpa_impl.allocator();

    const allow_private = parse_args(gpa);
    loop.run(allow_private) catch |e| {
        if (builtin.mode == .Debug) {
            return e;
        } else {
            die("{}", .{e});
        }
    };
}

fn parse_args(gpa: Allocator) bool {
    var arena_impl = std.heap.ArenaAllocator.init(gpa);
    defer arena_impl.deinit();
    const arena = arena_impl.allocator();

    const args = process.argsAlloc(arena) catch |e| {
        die("Failed to parse args. Error: {any}", .{e});
    };

    if (args.len > 1) {
        if (mem.eql(u8, args[1], "--allow-private")) {
            return true;
        } else {
            print_usage(args[0]);
            die("Unknown CLI argument \"{s}\"", .{args[1]});
        }
    }

    return false;
}

fn print_usage(progname: []const u8) void {
    const out = std.io.getStdOut().writer();
    out.print("Usage: {s} [--allow-private]\n", .{progname}) catch |e| {
        die("Failed to write to stdout: {any}", .{e});
    };
}

fn die(comptime fmt: []const u8, args: anytype) noreturn {
    std.log.err(fmt, args);
    process.exit(1);
}
