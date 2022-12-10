const DISPLAY_WIDTH: usize = 240;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu<'a> {
    x: i32,
    instructions: &'a [Instruction],
    mid_add: Option<i32>,
}

impl<'a> Cpu<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            x: 1,
            instructions,
            mid_add: None,
        }
    }

    fn advance(&mut self) -> bool {
        if let Some(i) = self.mid_add.take() {
            self.x += i;
            return self.instructions.is_empty();
        }
        let (&inst, rest) = self.instructions.split_first().unwrap();
        self.instructions = rest;
        match inst {
            Instruction::Noop => {}
            Instruction::Addx(i) => self.mid_add = Some(i),
        }
        self.instructions.is_empty()
    }
}

pub fn generator(s: &str) -> Vec<Instruction> {
    let mut result = Vec::with_capacity(1024);

    for line in s.lines() {
        result.push(if line == "noop" {
            Instruction::Noop
        } else {
            Instruction::Addx(line.strip_prefix("addx ").unwrap().parse().unwrap())
        });
    }

    result
}

pub fn part_1(instructions: &[Instruction]) -> i32 {
    let mut cpu = Cpu::new(instructions);

    for _ in 0..19 {
        cpu.advance();
    }
    let mut x_values_sum = 20 * cpu.x;

    for i in 0..5 {
        for _ in 0..40 {
            cpu.advance();
        }
        x_values_sum += cpu.x * ((i + 1) * 40 + 20);
    }

    x_values_sum
}

pub fn part_2(instructions: &[Instruction]) -> String {
    // 6 rows, with a newline
    let mut result = String::with_capacity((DISPLAY_WIDTH + 1) * 6);

    let mut cpu = Cpu::new(instructions);

    for _ in 0..6 {
        result.push('\n');
        for x in 0..40 {
            let ch = if cpu.x.abs_diff(x) < 2 { '#' } else { '.' };
            result.push(ch);
            cpu.advance();
        }
    }

    result
}

super::day_test! {demo_1 == 13140}
super::day_test! {demo_2 == "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."}
super::day_test! {part_1 == 13720}
super::day_test! {part_2 == "
####.###..#..#.###..#..#.####..##..#..#.
#....#..#.#..#.#..#.#..#....#.#..#.#..#.
###..###..#..#.#..#.####...#..#....####.
#....#..#.#..#.###..#..#..#...#....#..#.
#....#..#.#..#.#.#..#..#.#....#..#.#..#.
#....###...##..#..#.#..#.####..##..#..#."}
