const std = @import("std");
const zap = @import("zap");

pub fn main() !void {
    const currentDir = std.fs.cwd();
    const publicDir = try currentDir.makeOpenPath("public", .{});

    try currentDir.copyFile("index.html", publicDir, "index.html", .{});
    try currentDir.copyFile("zig-out/lib/main.wasm", publicDir, "main.wasm", .{});

    var listener = zap.HttpListener.init(.{
        .port = 8000,
        .on_request = struct {
            fn on_request(r: zap.Request) void {
                r.setStatus(.not_found);
            }
        }.on_request,
        .public_folder = "public",
        .log = true,
    });
    try listener.listen();

    std.debug.print("\nServing Zig experiment...\n", .{});

    zap.start(.{
        .threads = 2,
        .workers = 2,
    });
}
