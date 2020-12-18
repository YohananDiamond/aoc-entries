mod aoc;

use interpreter::{Instruction, Interpreter};

#[derive(Clone, Copy)]
pub enum PartConfig {
    Part1,
    Part2,
}

fn main() {
    aoc::start("day14_example1.txt", part1, aoc::dummy_part);
    aoc::start("day14_example2.txt", aoc::dummy_part, part2);
    aoc::start("day14.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let instructions: Vec<Instruction> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::new(line, PartConfig::Part1))
        .collect::<Result<_, _>>()?;

    let mut interpreter = Interpreter::new(&instructions);
    interpreter.run(PartConfig::Part1);

    Ok(format!(
        "{}",
        interpreter
            .memory_iter()
            .fold(0, |prev, (_addr, value)| prev + value)
    ))
}

fn part2(input: &str) -> Result<String, String> {
    let instructions: Vec<Instruction> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::new(line, PartConfig::Part2))
        .collect::<Result<_, _>>()?;

    // instructions
    //     .iter()
    //     .for_each(|instruction| match instruction {
    //         Instruction::SetMasks { masks } => {
    //             println!("  SET MASKS =");
    //             masks.iter().for_each(|mask| {
    //                 println!("    {}", mask.part2_repr());
    //             });
    //         }
    //         Instruction::ModifyAddress { address, value } => {
    //             println!("  SET &{} = {}", address, value);
    //         }
    //     });

    let mut interpreter = Interpreter::new(&instructions);
    interpreter.run(PartConfig::Part2);

    Ok(format!(
        "{}",
        interpreter
            .memory_iter()
            .fold(0, |prev, (_addr, value)| prev + value)
    ))
}

mod interpreter {
    use super::aoc::parse_number;
    use super::bit_tools::BitMask;
    use super::expect_match;
    use super::PartConfig;

    use std::collections::HashMap;

    pub type Address = usize;
    pub type Value = usize;

    #[derive(Debug, Clone)]
    pub struct Interpreter<'a> {
        instructions: &'a [Instruction],
        memory: HashMap<Address, Value>,
        masks: Vec<BitMask>,
    }

    #[derive(Debug, Clone)]
    pub enum Instruction {
        SetMasks { masks: Vec<BitMask> },
        ModifyAddress { address: usize, value: usize },
    }

    impl<'a> Interpreter<'a> {
        pub fn new(instructions: &'a [Instruction]) -> Self {
            Self {
                instructions: instructions,
                memory: HashMap::new(),
                masks: vec![BitMask::default()],
            }
        }

        pub fn run(&mut self, part: PartConfig) {
            for instruction in self.instructions.iter() {
                match instruction {
                    Instruction::SetMasks { masks } => self.masks = masks.clone(),
                    Instruction::ModifyAddress { address, value } => {
                        let apply = match part {
                            PartConfig::Part1 => BitMask::part1_apply,
                            PartConfig::Part2 => BitMask::part2_apply,
                        };

                        for mask in &self.masks {
                            apply(mask, &mut self.memory, *address, *value);
                        }
                    }
                }
            }
        }

        pub fn memory_iter(&self) -> impl Iterator<Item = (Address, Value)> + '_ {
            self.memory.iter().map(|(&a, &v)| (a, v))
        }
    }

    impl Instruction {
        pub fn new(data: &str, part: PartConfig) -> Result<Self, String> {
            let mut split = data.split(" ").peekable();

            match split.peek() {
                Some(&"mask") => {
                    split.next();
                    expect_match!(split.next(), Some("="))?;

                    let mask_str = expect_match!(split.next(), Some(_))?.unwrap();

                    Ok(Instruction::SetMasks {
                        masks: match part {
                            PartConfig::Part1 => vec![BitMask::part1_new(mask_str)?],
                            PartConfig::Part2 => BitMask::part2_new(mask_str)?,
                        },
                    })
                }
                Some(&other) if other.starts_with("mem[") => {
                    let mem_word = split.next().unwrap().chars().skip(4); // Format: /\d+\]/

                    let address = parse_number(
                        &mem_word
                            .clone()
                            .take_while(|c| c.is_digit(10))
                            .collect::<String>(),
                    )?;
                    let mut mem_word = mem_word.skip_while(|c| c.is_digit(10));

                    expect_match!(mem_word.next(), Some(']'))?;
                    expect_match!(mem_word.next(), None)?;

                    expect_match!(split.next(), Some("="))?;

                    let value = parse_number(&expect_match!(split.next(), Some(_))?.unwrap())?;

                    Ok(Instruction::ModifyAddress { address, value })
                }
                Some(&invalid) => {
                    return Err(format!(r#"Expected "mem[" or "mask", found {:?}"#, invalid))
                }
                None => return Err(format!("Input string is empty")),
            }
        }
    }
}

mod bit_tools {
    use super::interpreter::{Address, Value};
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct BitMask {
        /// The actual commands.
        ///
        /// The offset is supposed to go forward, starting from the 1th bit to the 36th bit.
        ///
        /// The length of this field is assured to be 36.
        bits: Vec<BitCommand>,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum BitCommand {
        SetOne,
        SetZero,
        DoNothing,
    }

    impl BitMask {
        pub fn part1_new(input: &str) -> Result<Self, String> {
            if input.len() != 36 {
                return Err(format!(
                    "Bitmask length should be 36 (found {})",
                    input.len()
                ));
            }

            Ok(Self {
                bits: input
                    .chars()
                    .rev()
                    .map(|c| match c {
                        '1' => Ok(BitCommand::SetOne),
                        '0' => Ok(BitCommand::SetZero),
                        'X' => Ok(BitCommand::DoNothing),
                        c => Err(format!("Unknown meaning for character: {:?}", c)),
                    })
                    .collect::<Result<_, _>>()?,
            })
        }

        pub fn part2_new(input: &str) -> Result<Vec<Self>, String> {
            if input.len() != 36 {
                return Err(format!(
                    "Bitmask length should be 36 (found {})",
                    input.len()
                ));
            }

            let mut result = vec![Vec::new()];

            for c in input.chars().rev() {
                match c {
                    '1' => result
                        .iter_mut()
                        .for_each(|bits| bits.push(BitCommand::SetOne)),
                    '0' => result
                        .iter_mut()
                        .for_each(|bits| bits.push(BitCommand::DoNothing)),
                    'X' => {
                        // get a new vector with double the capacity
                        let mut vec = Vec::with_capacity(result.len() * 2);

                        // for each `bits` on the old vector, get it and push into the new vector a version with SetZero and another with SetOne
                        while result.len() != 0 {
                            let mut bits_0 = result.remove(0);
                            let mut bits_1 = bits_0.clone();

                            bits_0.push(BitCommand::SetZero);
                            bits_1.push(BitCommand::SetOne);

                            vec.push(bits_0);
                            vec.push(bits_1);
                        }

                        std::mem::swap(&mut result, &mut vec);
                    }
                    c => return Err(format!("Unknown meaning for character: {:?}", c)),
                }
            }

            Ok(result.into_iter().map(|bits| Self { bits }).collect())
        }

        #[allow(dead_code)]
        pub fn part1_repr(&self) -> String {
            self.bits
                .iter()
                .rev()
                .map(|command| match command {
                    BitCommand::SetZero => '0',
                    BitCommand::SetOne => '1',
                    BitCommand::DoNothing => 'X',
                })
                .collect()
        }

        #[allow(dead_code)]
        pub fn part2_repr(&self) -> String {
            self.bits
                .iter()
                .rev()
                .map(|command| match command {
                    BitCommand::SetZero => 'Z',
                    BitCommand::SetOne => '1',
                    BitCommand::DoNothing => '0',
                })
                .collect()
        }

        pub fn part1_apply(
            &self,
            memory: &mut HashMap<Address, Value>,
            address: Address,
            mut value: Value,
        ) {
            for (i, &bitcmd) in self.bits.iter().enumerate() {
                match bitcmd {
                    BitCommand::SetOne => value |= 1 << i,
                    BitCommand::SetZero => value &= !(1 << i),
                    BitCommand::DoNothing => {}
                }
            }

            memory.insert(address, value);
        }

        pub fn part2_apply(
            &self,
            memory: &mut HashMap<Address, Value>,
            mut address: Address,
            value: Value,
        ) {
            for (i, &bitcmd) in self.bits.iter().enumerate() {
                match bitcmd {
                    BitCommand::SetOne => address |= 1 << i,
                    BitCommand::SetZero => address &= !(1 << i),
                    BitCommand::DoNothing => {}
                }
            }

            memory.insert(address, value);
        }
    }

    impl From<[BitCommand; 36]> for BitMask {
        fn from(array: [BitCommand; 36]) -> Self {
            Self {
                bits: Vec::from(array),
            }
        }
    }

    impl Default for BitMask {
        fn default() -> Self {
            BitMask::from([BitCommand::DoNothing; 36])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bit_tools::BitMask;
    use super::PartConfig;

    #[test]
    fn basic_part1_bitmask() {
        let bitmask =
            BitMask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX10X01", PartConfig::Part1).unwrap();
        let val = 0b0001111111100000;

        assert_eq!(bitmask.applied_on(val), 0b0001111111110001);
    }

    #[test]
    fn default_bitmask() {
        Bitmask::default();
    }
}
