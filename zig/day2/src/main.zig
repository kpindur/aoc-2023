const std = @import("std");
const Alloc = std.mem.Allocator;

const Colors = struct { red: usize, green: usize, blue: usize };
const Game = struct { turns: []Colors, limits: Colors };

fn readInputFile(allocator: Alloc, filename: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(filename, .{ .mode = .read_only });
    defer file.close();

    const stat = try file.stat();
    const fileSize = stat.size;

    return try file.reader().readAllAlloc(allocator, fileSize);
}

fn part1(allocator: Alloc, buff: []const u8) !usize {
    var lines = std.ArrayList([]const u8).init(allocator);
    defer lines.deinit();
    var result: usize = 0;
    var limits = Colors{ .red = 12, .green = 13, .blue = 14 };

    var readLines = std.mem.tokenizeAny(u8, buff, "\n");
    var i: usize = 1;
    while (readLines.next()) |line| {
        defer i += 1;
        var tokens = std.mem.split(u8, line, ": ");
        _ = tokens.next();
        var games = std.mem.split(u8, tokens.next().?, "; ");
        var colors = std.ArrayList(Colors).init(allocator);
        while (games.next()) |game| {
            var turns = std.mem.split(u8, game, ", ");
            var red: usize = 0;
            var green: usize = 0;
            var blue: usize = 0;
            while (turns.next()) |turn| {
                var hand = std.mem.split(u8, turn, " ");
                const value = hand.next().?;
                const color = hand.next().?;

                if (std.mem.eql(u8, "red", color)) red = try std.fmt.parseInt(usize, value, 10);
                if (std.mem.eql(u8, "green", color)) green = try std.fmt.parseInt(usize, value, 10);
                if (std.mem.eql(u8, "blue", color)) blue = try std.fmt.parseInt(usize, value, 10);
            }
            try colors.append(Colors{ .red = red, .green = green, .blue = blue });
        }
        var gameIsValid: bool = true;
        for (colors.items) |color| {
            const redIsValid = color.red <= limits.red;
            const greenIsValid = color.green <= limits.green;
            const blueIsValid = color.blue <= limits.blue;

            if (!redIsValid or !greenIsValid or !blueIsValid) {
                gameIsValid = false;
                break;
            }
        }
        if (gameIsValid) result += i;
    }
    return result;
}

fn part2(allocator: Alloc, buff: []const u8) !usize {
    var lines = std.ArrayList([]const u8).init(allocator);
    defer lines.deinit();
    var result: usize = 0;
    var limits = Colors{ .red = 12, .green = 13, .blue = 14 };
    _ = limits;

    var readLines = std.mem.tokenizeAny(u8, buff, "\n");
    var i: usize = 1;
    _ = i;
    while (readLines.next()) |line| {
        var tokens = std.mem.split(u8, line, ": ");
        _ = tokens.next();
        var games = std.mem.split(u8, tokens.next().?, "; ");
        var colors = std.ArrayList(Colors).init(allocator);
        while (games.next()) |game| {
            var turns = std.mem.split(u8, game, ", ");
            var red: usize = 0;
            var green: usize = 0;
            var blue: usize = 0;
            while (turns.next()) |turn| {
                var hand = std.mem.split(u8, turn, " ");
                const value = hand.next().?;
                const color = hand.next().?;

                if (std.mem.eql(u8, "red", color)) red = try std.fmt.parseInt(usize, value, 10);
                if (std.mem.eql(u8, "green", color)) green = try std.fmt.parseInt(usize, value, 10);
                if (std.mem.eql(u8, "blue", color)) blue = try std.fmt.parseInt(usize, value, 10);
            }
            try colors.append(Colors{ .red = red, .green = green, .blue = blue });
        }
        var red_max: usize = 0;
        var green_max: usize = 0;
        var blue_max: usize = 0;

        for (colors.items) |color| {
            if (color.red > red_max) red_max = color.red;
            if (color.green > green_max) green_max = color.green;
            if (color.blue > blue_max) blue_max = color.blue;
        }
        result += red_max * green_max * blue_max;
    }
    return result;
}

pub fn main() !void {
    const filename = "./src/data/input.dat";
    const allocator = std.heap.page_allocator;

    const buff = try readInputFile(allocator, filename);
    const result1 = try part1(allocator, buff);
    const result2 = try part2(allocator, buff);

    std.debug.print("Part1: {}\nPart2: {}\n", .{ result1, result2 });
}

test "part1" {
    const buff = @embedFile("./data/test.dat");
    const allocator = std.heap.page_allocator;

    const result = try part1(allocator, buff);
    std.debug.assert(8 == result);
}

test "part2" {
    const buff = @embedFile("./data/test.dat");
    const allocator = std.heap.page_allocator;

    const result = try part2(allocator, buff);
    std.debug.assert(2286 == result);
}
