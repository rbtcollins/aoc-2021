use std::fmt::Display;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

pub fn generate(input: &str) -> impl '_ + Clone + Iterator<Item = Instruction> {
    input.lines().map(|l| match l {
        "noop" => Instruction::Noop,
        s if s.starts_with("addx ") => Instruction::Addx(str::parse(s.split_at(5).1).unwrap()),
        _v => unreachable!("{_v:?}"),
    })
}

pub fn part_1(input: &(impl Clone + Iterator<Item = Instruction>)) -> usize {
    simulate_cpu(input)
        .flatten()
        .flatten()
        .fold(0, |mut ss, cpu| {
            if cpu.cc % 40 == 20 {
                ss += cpu.x * cpu.cc;
            };
            ss
        }) as usize
}

#[derive(Clone, Copy, Debug)]
struct Cpu {
    cc: isize,
    x: isize,
}

fn simulate_cpu(
    input: &(impl Clone + Iterator<Item = Instruction>),
) -> impl Iterator<Item = [Option<Cpu>; 2]> {
    [[Some(Cpu { x: 0, cc: 1 }), None]]
        .into_iter()
        .chain(input.clone().scan(
            Cpu { cc: 1, x: 1 },
            |state, instruction| match instruction {
                Instruction::Noop => {
                    state.cc += 1;
                    Some([Some(*state), None])
                }
                Instruction::Addx(v) => {
                    let before = Cpu {
                        cc: state.cc + 1,
                        x: state.x,
                    };
                    state.x += v;
                    state.cc += 2;
                    Some([Some(before), Some(*state)])
                }
            },
        ))
}

#[derive(Clone, Copy, Debug)]
pub struct Crt([[char; 40]; 6]);
impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn part_2(input: &(impl Clone + Iterator<Item = Instruction>)) -> Crt {
    simulate_cpu(input)
        .flatten()
        .flatten()
        .take(240)
        .fold(Crt([[' '; 40]; 6]), |mut crt, cpu| {
            let pixel = (cpu.cc - 1) % 40;
            let row = ((cpu.cc - 1) as usize / 40) % 6;
            crt.0[row][pixel as usize] = if (pixel - cpu.x).abs() < 2 { '#' } else { '.' };
            crt
        })
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(13140, super::part_1(&input));
    }

    #[test]
    fn part_2() {
        let input = super::generate(SAMPLE);
        let crt = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(crt, super::part_2(&input).to_string());
    }
}
