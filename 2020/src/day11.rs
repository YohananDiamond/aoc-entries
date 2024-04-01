mod aoc;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
}

fn main() {
    aoc::start("day11_example.txt", part1, part2);
    aoc::start("day11.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let mut grid = Grid::new(input)?;
    while grid.process(Grid::part1_occupied_adjacent_count, 4) != 0 {}

    Ok(format!("{}", grid.occupied_seats_count()))
}

fn part2(input: &str) -> Result<String, String> {
    let mut grid = Grid::new(input)?;
    while grid.process(Grid::part2_occupied_adjacent_count, 5) != 0 {}

    Ok(format!("{}", grid.occupied_seats_count()))
}

impl Tile {
    pub fn new(c: char) -> Result<Self, String> {
        match c {
            'L' => Ok(Self::EmptySeat),
            '#' => Ok(Self::OccupiedSeat),
            '.' => Ok(Self::Floor),
            c => Err(format!("Unknown character: {:?}", c)),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::OccupiedSeat => '#',
            Tile::EmptySeat => 'L',
            Tile::Floor => '.',
        }
    }
}

impl Grid {
    pub fn new(input: &str) -> Result<Self, String> {
        let split = input.split("\n").filter(|line| !line.is_empty());
        let width = split
            .clone()
            .next()
            .ok_or_else(|| format!("Input is empty"))?
            .len();

        Ok(Self {
            tiles: split
                .map(|line| {
                    if line.len() == width {
                        Ok(line.chars().map(Tile::new).collect::<Result<Vec<_>, _>>()?)
                    } else {
                        Err(format!(
                            "Invalid length ({}, expected {}) on line {:?}",
                            line.len(),
                            width,
                            line
                        ))
                    }
                })
                .collect::<Result<_, _>>()?,
            width: width,
        })
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn occupied_seats_count(&self) -> usize {
        self.tiles.iter().fold(0, |result, row| {
            result
                + row.iter().fold(0, |result, tile| {
                    result + if let Tile::OccupiedSeat = tile { 1 } else { 0 }
                })
        })
    }

    /// Goes through a round of people moving and returns how many seats changed.
    pub fn process<F>(&mut self, mut adjacent_fn: F, tolerance_level: usize) -> usize
    where
        F: FnMut(&Self, usize, usize) -> usize,
    {
        // construct the new grid
        let mut here: Vec<Vec<_>> = self
            .tiles
            .iter()
            .enumerate()
            .map(|(ir, row)| {
                row.iter()
                    .enumerate()
                    .map(|(ic, column)| match column {
                        Tile::EmptySeat => {
                            if adjacent_fn(self, ir, ic) == 0 {
                                Tile::OccupiedSeat
                            } else {
                                Tile::EmptySeat
                            }
                        }
                        Tile::OccupiedSeat => {
                            if adjacent_fn(self, ir, ic) >= tolerance_level {
                                Tile::EmptySeat
                            } else {
                                Tile::OccupiedSeat
                            }
                        }
                        Tile::Floor => Tile::Floor,
                    })
                    .collect()
            })
            .collect();

        // swap with the old grid
        std::mem::swap(&mut here, &mut self.tiles);

        // return the difference
        self.seat_difference(&here)
    }

    fn seat_difference(&self, other: &Vec<Vec<Tile>>) -> usize {
        self.tiles.iter().enumerate().fold(0, |result, (ir, row)| {
            result
                + row.iter().enumerate().fold(0, |result, (ic, column)| {
                    result + if other[ir][ic] != *column { 1 } else { 0 }
                })
        })
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&Tile> {
        self.tiles.get(row)?.get(column)
    }

    fn part1_occupied_adjacent_count(&self, row: usize, column: usize) -> usize {
        [
            // north
            if row != 0 {
                self.get(row - 1, column)
            } else {
                None
            },
            // north-east
            if row != 0 {
                self.get(row - 1, column + 1)
            } else {
                None
            },
            // east
            self.get(row, column + 1),
            // south-east
            self.get(row + 1, column + 1),
            // south
            self.get(row + 1, column),
            // south-west
            if column != 0 {
                self.get(row + 1, column - 1)
            } else {
                None
            },
            // west
            if column != 0 {
                self.get(row, column - 1)
            } else {
                None
            },
            // north-west
            if column != 0 && row != 0 {
                self.get(row - 1, column - 1)
            } else {
                None
            },
        ]
        .iter()
        .fold(0, |result, seat| {
            result
                + if let Some(Tile::OccupiedSeat) = seat {
                    1
                } else {
                    0
                }
        })
    }

    fn part2_occupied_adjacent_count(&self, row: usize, column: usize) -> usize {
        fn find_occupied_seat(
            grid: &Vec<Vec<Tile>>,
            starting_point: (isize, isize),
            direction: (isize, isize),
        ) -> bool {
            let (mut prow, mut pcol) = starting_point;

            loop {
                prow += direction.0;
                pcol += direction.1;

                if prow < 0 || pcol < 0 {
                    return false;
                }

                if let Some(row) = grid.get(prow as usize) {
                    match row.get(pcol as usize) {
                        Some(Tile::OccupiedSeat) => return true,
                        Some(Tile::EmptySeat) => return false,
                        None => return false,
                        _ => {}
                    }
                } else {
                    return false;
                }
            }
        }

        [
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (-1, 0)),
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (-1, 1)),
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (0, 1)),
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (1, 1)),
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (1, 0)),
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (1, -1)),
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (0, -1)),
            find_occupied_seat(&self.tiles, (row as isize, column as isize), (-1, -1)),
        ]
        .iter()
        .fold(0, |result, &this| result + if this { 1 } else { 0 })
    }

    #[allow(dead_code)]
    pub fn show_graphical_representation(&self) {
        for row in &self.tiles {
            println!(
                "{}",
                row.iter()
                    .map(|&column| <Tile as Into<char>>::into(column))
                    .collect::<String>()
            );
        }
    }
}
