// café: declaration
const std = @import("std");
pub fn main() void {
    const count: u32 = 42;
    std.debug.print("hello {d}\n", .{count});
}
