mod aoc;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    GoNorth(f64),
    GoSouth(f64),
    GoWest(f64),
    GoEast(f64),
    TurnLeft(f64),
    TurnRight(f64),
    GoForward(f64),
}

fn main() {
    aoc::start("day12_example.txt", part1, part2);
    aoc::start("day12.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let instructions = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::new(line).map_err(|e| format!("While parsing {:?}: {}", line, e)))
        .collect::<Result<Vec<_>, _>>()?;

    let (fpx, fpy) = process_instructions_part1(&instructions, (0.0, 0.0));

    Ok(format!("{}", fpx.abs() + fpy.abs()))
}

fn part2(input: &str) -> Result<String, String> {
    let instructions = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::new(line).map_err(|e| format!("While parsing {:?}: {}", line, e)))
        .collect::<Result<Vec<_>, _>>()?;

    let (fpx, fpy) = process_instructions_part2(&instructions, (0.0, 0.0));

    Ok(format!("{}", fpx.abs() + fpy.abs()))
}

impl Instruction {
    pub fn new(string: &str) -> Result<Self, String> {
        if let Some(c) = string.chars().next() {
            let num_string = &string[1..];
            let num = num_string
                .parse()
                .map_err(|e| format!("Failed to parse number string {:?}: {}", num_string, e))?;

            match c {
                'N' => Ok(Instruction::GoNorth(num)),
                'S' => Ok(Instruction::GoSouth(num)),
                'W' => Ok(Instruction::GoWest(num)),
                'E' => Ok(Instruction::GoEast(num)),
                'L' => Ok(Instruction::TurnLeft(num)),
                'R' => Ok(Instruction::TurnRight(num)),
                'F' => Ok(Instruction::GoForward(num)),
                c => Err(format!(
                    "Unknown matching instruction for character {:?}",
                    c
                )),
            }
        } else {
            Err(format!("Input string is empty"))
        }
    }
}

fn process_instructions_part1(
    instructions: &[Instruction],
    starting_pos: (f64, f64),
) -> (f64, f64) {
    let (mut px, mut py) = starting_pos;
    let mut angle: f64 = 0.0;

    for instruction in instructions {
        match instruction {
            Instruction::GoNorth(num) => py += num,
            Instruction::GoSouth(num) => py -= num,
            Instruction::GoWest(num) => px -= num,
            Instruction::GoEast(num) => px += num,
            Instruction::TurnLeft(ang) => angle += ang,
            Instruction::TurnRight(ang) => angle -= ang,
            Instruction::GoForward(num) => {
                px += num * (angle.to_radians().cos() as i64) as f64;
                py += num * (angle.to_radians().sin() as i64) as f64;
            }
        }
    }

    (px, py)
}

fn process_instructions_part2(
    instructions: &[Instruction],
    starting_pos: (f64, f64),
) -> (f64, f64) {
    let (mut px, mut py) = starting_pos;
    let (mut wx, mut wy) = (10.0, 1.0);

    for instruction in instructions {
        // println!("P{:?} W{:?}, INSTRUCTION {:?}", (px, py), (wx, wy), instruction);

        match instruction {
            Instruction::GoNorth(num) => wy += num,
            Instruction::GoSouth(num) => wy -= num,
            Instruction::GoWest(num) => wx -= num,
            Instruction::GoEast(num) => wx += num,
            Instruction::TurnLeft(mut ang) => {
                assert!(ang % 90.0 == 0.0);

                while ang > 0.0 {
                    let nwx = -wy;
                    let nwy = wx;

                    wx = nwx;
                    wy = nwy;

                    ang -= 90.0;
                }
            }
            Instruction::TurnRight(mut ang) => {
                assert!(ang % 90.0 == 0.0);

                while ang > 0.0 {
                    let nwx = wy;
                    let nwy = -wx;

                    wx = nwx;
                    wy = nwy;

                    ang -= 90.0;
                }
            }
            Instruction::GoForward(num) => {
                px += wx * num;
                py += wy * num;
            }
        }

        // {
        // let new_angle = wy.atan2(wx) + ang;
        // let intensity = (wx.powf(2.0) + wy.powf(2.0)).sqrt();

        // wx = intensity * new_angle.cos();
        // wy = intensity * new_angle.sin();
        // }
    }

    (px, py)
}
