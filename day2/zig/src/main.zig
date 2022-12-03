const std = @import("std");

const INPUT_FILE = "../input.txt";

const Ordering = enum {
    Greater,
    Equal,
    Less,
};

const GameOutcome = enum {
    Win,
    Lose,
    Draw,

    fn from_char(char: u8) GameOutcome {
        return switch (char) {
            'X' => .Lose,
            'Y' => .Draw,
            'Z' => .Win,
            else => unreachable,
        };
    }
};

const Hand = enum {
    Rock,
    Paper,
    Scissors,

    fn from_char(char: u8) Hand {
        return switch (char) {
            'A', 'X' => .Rock,
            'B', 'Y' => .Paper,
            'C', 'Z' => .Scissors,
            else => unreachable,
        };
    }

    fn play(self: Hand, other: Hand) u32 {
        return self.score() + self.outcome_score(other);
    }

    fn outcome_score(self: Hand, other: Hand) u32 {
        return switch (self.cmp(other)) {
            .Greater => 6,
            .Equal => 3,
            .Less => 0,
        };
    }

    fn pivot_hand(self: Hand, outcome: GameOutcome) Hand {
        return switch (self) {
            .Rock => switch (outcome) {
                .Win => .Paper,
                .Draw => .Rock,
                .Lose => .Scissors,
            },
            .Paper => switch (outcome) {
                .Win => .Scissors,
                .Draw => .Paper,
                .Lose => .Rock,
            },
            .Scissors => switch (outcome) {
                .Win => .Rock,
                .Draw => .Scissors,
                .Lose => .Paper,
            },
        };
    }

    fn score(self: Hand) u32 {
        return switch (self) {
            .Rock => 1,
            .Paper => 2,
            .Scissors => 3,
        };
    }

    fn cmp(self: Hand, other: Hand) Ordering {
        return switch (self) {
            .Rock => Hand.cmp_rock(other),
            .Paper => Hand.cmp_paper(other),
            .Scissors => Hand.cmp_scissors(other),
        };
    }

    fn cmp_rock(other: Hand) Ordering {
        return switch (other) {
            .Rock => .Equal,
            .Paper => .Less,
            .Scissors => .Greater,
        };
    }

    fn cmp_paper(other: Hand) Ordering {
        return switch (other) {
            .Rock => .Greater,
            .Paper => .Equal,
            .Scissors => .Less,
        };
    }

    fn cmp_scissors(other: Hand) Ordering {
        return switch (other) {
            .Rock => .Less,
            .Paper => .Greater,
            .Scissors => .Equal,
        };
    }
};

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

    {
        var score: u32 = 0;
        var iter_line = std.mem.split(u8, input_buffer, "\n");
        while (iter_line.next()) |line| {
            const opponent = Hand.from_char(line[0]);
            const you = Hand.from_char(line[2]);
            score += you.play(opponent);
        }
        std.log.info("total score: {d}", .{score});
    }

    {
        var score: u32 = 0;
        var iter_line = std.mem.split(u8, input_buffer, "\n");
        while (iter_line.next()) |line| {
            const opponent = Hand.from_char(line[0]);
            const outcome = GameOutcome.from_char(line[2]);
            const you = opponent.pivot_hand(outcome);
            score += you.play(opponent);
        }
        std.log.info("total score: {d}", .{score});
    }
}
