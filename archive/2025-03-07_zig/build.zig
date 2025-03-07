const std = @import("std");
const Target = std.Target;

const optimize = std.builtin.Mode.Debug;

// Although this function looks imperative, note that its job is to
// declaratively construct a build graph that will be executed by an external
// runner.
pub fn build(b: *std.Build) void {
    buildAndInstallWasm(b);
    buildAndRunServer(b);
    buildAndRunTests(b);
}

fn buildAndInstallWasm(b: *std.Build) void {
    const target = b.standardTargetOptions(.{ .default_target = .{ .cpu_arch = Target.Cpu.Arch.wasm32, .os_tag = Target.Os.Tag.freestanding } });

    const lib = b.addSharedLibrary(.{
        .name = "main",
        // In this case the main source file is merely a path, however, in more
        // complicated build scripts, this could be a generated file.
        .root_source_file = .{ .path = "src/wasm/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    lib.rdynamic = true;

    // This declares intent for the library to be installed into the standard
    // location when the user invokes the "install" step (the default step when
    // running `zig build`).
    b.installArtifact(lib);
}

fn buildAndRunServer(b: *std.Build) void {
    const server = b.addExecutable(.{
        .name = "server",
        .root_source_file = .{ .path = "src/server/main.zig" },
        .target = .{},
        .optimize = optimize,
    });

    const zap = b.dependency("zap", .{
        .target = std.zig.CrossTarget{},
        .optimize = optimize,
        .openssl = false,
    });
    server.addModule("zap", zap.module("zap"));
    server.linkLibrary(zap.artifact("facil.io"));

    b.installArtifact(server);

    const run_cmd = b.addRunArtifact(server);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run the server");
    run_step.dependOn(&run_cmd.step);
}

fn buildAndRunTests(b: *std.Build) void {
    // Creates a step for unit testing. This only builds the test executable
    // but does not run it.
    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/wasm/main.zig" },
        .target = .{},
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);

    // This creates a build step. It will be visible in the `zig build --help` menu,
    // and can be selected like this: `zig build test`
    // This will evaluate the `test` step rather than the default, which is "install".
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);
}
