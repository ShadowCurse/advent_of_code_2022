const std = @import("std");
const array = std.ArrayList(u32);

const INPUT_FILE = "../input.txt";

pub fn main() !void {
    var gp = std.heap.GeneralPurposeAllocator(.{ .safety = true }){};
    defer _ = gp.deinit();
    const allocator = gp.allocator();

    var input = try std.fs.cwd().openFile(INPUT_FILE, .{});
    defer input.close();

    var input_meta = try input.metadata();
    var input_size = input_meta.size();

    const input_buffer = try input.readToEndAlloc(allocator, input_size);
    defer allocator.free(input_buffer);

    var bags_vals = array.init(allocator);
    defer bags_vals.deinit();

    var iter_bags = std.mem.split(u8, input_buffer, "\n\n");
    while (iter_bags.next()) |bag| {
        var sum: u32 = 0;
        var iter_bag_items = std.mem.split(u8, bag, "\n");
        while (iter_bag_items.next()) |item| {
            const item_val = std.fmt.parseInt(u32, item, 10) catch unreachable;
            sum += item_val;
        }
        try bags_vals.append(sum);
    }

    std.sort.sort(u32, bags_vals.allocatedSlice(), {}, comptime std.sort.asc(u32));

    const max_bag = bags_vals.items[bags_vals.items.len - 1];

    var sum_most_3: u32 = 0;
    for (bags_vals.items[bags_vals.items.len - 3 ..]) |value| {
        sum_most_3 += value;
    }

    std.log.info("Max bag: {d}", .{max_bag});
    std.log.info("Sum most 3: {d}", .{sum_most_3});
}
