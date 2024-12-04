mod day1;
mod day2;
mod day3;

aoc_main::main! {
    year 2024;
    day1 : generate => part_1, part_2;
    day2 : generate => part_1, part_2;
    day3 : generate => part_1, part_2, part1_fold, part2_fold;
}
