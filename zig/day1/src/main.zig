const std = @import("std");
const Allocator = std.mem.Allocator;
const print = std.debug.print;

fn readFile(allocator: Allocator, filename: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(
        filename,
        .{ .mode = .read_only },
    );
    defer file.close();

    const stat = try file.stat();
    var buff = try file.readToEndAlloc(allocator, stat.size);
    return buff;
}

fn isDigit(c: u8) bool {
    return c >= '0' and c <= '9';
}

fn part1(buff: []const u8) usize {
    var lines = std.mem.split(u8, buff, "\n");
    var result: usize = 0;

    while (lines.next()) |line| {
        if (line.len == 0) break;

        var first_digit: ?u8 = null;
        var last_digit: ?u8 = null;

        for (line) |c| {
            if (isDigit(c)) {
                if (first_digit == null) first_digit = c;
                last_digit = c;
            }
        }

        if (first_digit != null and last_digit != null) {
            result += (first_digit.? - '0') * 10 + (last_digit.? - '0');
        }
    }

    return result;
}

fn findAllOccurences(allocator: Allocator, haystack: []const u8, needle: []const u8) *std.ArrayList(usize) {
    var indices = std.ArrayList(usize).init(allocator);

    var i: usize = 0;
    while (i < haystack.len) {
        if (std.mem.startsWith(u8, haystack[i..], needle)) {
            indices.append(i) catch unreachable;
            i += needle.len;
        } else {
            i += 1;
        }
    }

    return &indices;
}

fn part2(allocator: Allocator, buff: []const u8) !usize {
    const substrings = [_][]const u8{ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9" };
    var map = std.StringHashMap(usize).init(allocator);
    defer map.deinit();

    try map.put("one", 1);
    try map.put("two", 2);
    try map.put("three", 3);
    try map.put("four", 4);
    try map.put("five", 5);
    try map.put("six", 6);
    try map.put("seven", 7);
    try map.put("eight", 8);
    try map.put("nine", 9);

    try map.put("1", 1);
    try map.put("2", 2);
    try map.put("3", 3);
    try map.put("4", 4);
    try map.put("5", 5);
    try map.put("6", 6);
    try map.put("7", 7);
    try map.put("8", 8);
    try map.put("9", 9);

    var result: usize = 0;

    var lines = std.mem.split(u8, buff, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) break;

        var found_substrings = std.ArrayList([]const u8).init(allocator);
        var found_indices = std.ArrayList(usize).init(allocator);
        defer found_indices.deinit();
        defer found_substrings.deinit();

        for (substrings) |substring| {
            const indices = findAllOccurences(allocator, line, substring);
            for (indices.items) |index| {
                try found_substrings.append(substring);
                try found_indices.append(index);
            }
        }

        var min = found_indices.items[0];
        var max = found_indices.items[0];
        var min_index: usize = 0;
        var max_index: usize = 0;
        var i: usize = 0;
        for (found_indices.items) |val| {
            if (val < min) {
                min = val;
                min_index = i;
            }
            if (val > max) {
                max = val;
                max_index = i;
            }
            i += 1;
        }

        const first_digit = map.get(found_substrings.items[min_index]);
        const last_digit = map.get(found_substrings.items[max_index]);
        if (first_digit != null and last_digit != null) {
            result += (first_digit.?) * 10 + (last_digit.?);
        }
    }

    return result;
}

pub fn main() !void {
    const file_path = "input.dat";
    const allocator = std.heap.page_allocator;

    const buff = try readFile(allocator, file_path);
    defer allocator.free(buff);

    const result1 = part1(buff);
    const result2 = try part2(allocator, buff);
    print("Part1: {d}\nPart2: {d}\n", .{ result1, result2 });
}

test "part1" {
    const file_path = "test1.dat";
    const allocator = std.heap.page_allocator;

    const buff = try readFile(allocator, file_path);
    defer allocator.free(buff);

    const result = part1(buff);
    std.debug.assert(result == 142);
}

test "part2" {
    const file_path = "test2.dat";
    const allocator = std.heap.page_allocator;

    const buff = try readFile(allocator, file_path);
    defer allocator.free(buff);

    const result = try part2(allocator, buff);
    std.debug.assert(result == 281);
}
