mod aoc;

struct Emulator {
    bootloader: Vec<Instruction>,
    instruction_ptr: isize,
    pub accumulator: i32,
}

struct Instruction {
    run_counter: u32,
    opcode: Opcode,
    arg: i32,
}

#[derive(Clone, Copy)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

enum Response {
    Finished,
    RepeatedInstruction(isize),
}

fn main() {
    aoc::start("day8_example.txt", part1, part2);
    aoc::start("day8.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let mut emulator = Emulator::new(input)?;
    emulator.run()?;

    Ok(format!("{}", emulator.accumulator))
}

fn part2(input: &str) -> Result<String, String> {
    let mut emulator = Emulator::new(input)?;

    for i in 0..emulator.bootloader.len() {
        // efficiency! :D
        let true_opcode = unsafe { emulator.bootloader.get_unchecked(i).opcode };

        if let Opcode::Jmp | Opcode::Nop = true_opcode {
            unsafe fn swap(emulator: &mut Emulator, i: usize) {
                let opcode = emulator.bootloader.get_unchecked_mut(i).opcode;

                emulator.bootloader.get_unchecked_mut(i).opcode = match opcode {
                    Opcode::Jmp => Opcode::Nop,
                    Opcode::Nop => Opcode::Jmp,
                    other => other,
                };
            }

            unsafe { swap(&mut emulator, i) };

            if let Response::Finished = emulator.run()? {
                return Ok(format!("{}", emulator.accumulator));
            }

            emulator.reset();

            unsafe { swap(&mut emulator, i) };
        }
    }

    Err(format!("All attempts to replace jmp/nop failed."))
}

impl Emulator {
    pub fn new(input: &str) -> Result<Self, String> {
        Ok(Self {
            bootloader: input
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(|line| Instruction::new(line))
                .collect::<Result<Vec<_>, _>>()?,
            instruction_ptr: 0,
            accumulator: 0,
        })
    }

    /// Keeps running the program until a response is triggered.
    pub fn run(&mut self) -> Result<Response, String> {
        loop {
            if self.instruction_ptr < 0 {
                break Err(format!(
                    "Instruction pointer is below zero (@{})",
                    self.instruction_ptr
                ));
            }

            if let Some(Instruction {
                ref mut run_counter,
                opcode,
                arg,
            }) = self.bootloader.get_mut(self.instruction_ptr as usize)
            {
                if *run_counter != 0 {
                    break Ok(Response::RepeatedInstruction(self.instruction_ptr));
                }

                match opcode {
                    Opcode::Acc => {
                        self.accumulator += *arg;
                        self.instruction_ptr += 1;
                    }
                    Opcode::Jmp => {
                        self.instruction_ptr += *arg as isize;
                    }
                    Opcode::Nop => {
                        self.instruction_ptr += 1;
                    }
                }

                *run_counter += 1;
            } else {
                break Ok(Response::Finished);
            }
        }
    }

    pub fn reset(&mut self) {
        self.instruction_ptr = 0;
        self.accumulator = 0;

        for instruction in self.bootloader.iter_mut() {
            instruction.run_counter = 0;
        }
    }
}

impl Instruction {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut split = input.split(" ");
        let (opcode, arg) = (
            split
                .next()
                .ok_or_else(|| format!("Premature end of input (expected opcode)"))?,
            split
                .next()
                .ok_or_else(|| format!("Premature end of input (expected number)"))?,
        );

        let arg = arg
            .parse::<i32>()
            .map_err(|e| format!("Failed to parse number {:?}: {}", arg, e))?;

        Ok(Self {
            run_counter: 0,
            opcode: match opcode {
                "acc" => Opcode::Acc,
                "jmp" => Opcode::Jmp,
                "nop" => Opcode::Nop,
                opcode => return Err(format!("Unknown opcode {:?}", opcode)),
            },
            arg: arg,
        })
    }
}
