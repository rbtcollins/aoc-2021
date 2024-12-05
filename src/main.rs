mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

aoc_main::main! {
    year 2024;
    day1 : generate => part_1, part_2;
    day2 : generate => part_1, part_2;
    day3 : generate => part_1, part_2, part_1_fold, part_2_fold;
    day4 : generate => part_1, part_2, part_1_rayon, part_2_rayon;
    day5 : generate => part_1, part_2, part_1_rayon, part_2_rayon;
}
