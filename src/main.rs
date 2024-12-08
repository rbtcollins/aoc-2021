mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

aoc_main::main! {
    year 2024;
    day1 : generate => part_1, part_2;
    day2 : generate => part_1, part_2;
    day3 : generate => part_1, part_2, part_1_fold, part_2_fold;
    day4 : generate => part_1, part_2, part_1_rayon, part_2_rayon;
    day5 : generate => part_1, part_2, part_1_rayon, part_2_rayon;
    day6 : generate => part_1, part_2;
    day7 : generate => part_1, part_1_from_end, part_2, part_2_from_end;
    day8 : generate => part_1, part_2;
}
