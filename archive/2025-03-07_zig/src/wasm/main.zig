const std = @import("std");
const testing = std.testing;

const ffi = @import("ffi.zig");

export fn add(a: i32, b: i32) i32 {
    print("Hello, world!");

    return a + b;
}

fn print(message: []const u8) void {
    ffi.print(message.ptr, message.len);
}

test "basic add functionality" {
    try testing.expect(add(3, 7) == 10);
}
