mod aoc;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

#[derive(Debug, Clone)]
struct Dimension3 {
    /// A set which stores the coordinates of all active cubes.
    /// Inactive cubes simply are not represented here.
    active_cubes: HashSet<Point3>,
}

#[derive(Debug, Clone)]
struct Dimension4 {
    /// A set which stores the coordinates of all active cubes.
    /// Inactive cubes simply are not represented here.
    active_cubes: HashSet<Point4>,
}

fn main() {
    aoc::start_with_file("day17_example.txt", part1, part2);
    aoc::start_with_file("day17.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let mut dimension = Dimension3::with_initial_grid(input)?;
    (0..6).for_each(|_| dimension.perform_cycle());

    Err(format!("{}", dimension.active_cubes_amount()))
}

fn part2(input: &str) -> Result<String, String> {
    let mut dimension = Dimension4::with_initial_grid(input)?;
    (0..6).for_each(|_| dimension.perform_cycle());

    Err(format!("{}", dimension.active_cubes_amount()))
}

impl Point3 {
    #[inline]
    #[allow(dead_code)]
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn nearby_points(&self, offset: usize) -> impl Iterator<Item = Point3> + '_ {
        let offset = offset as isize;

        (-offset..=offset)
            .map(move |y| {
                (-offset..=offset)
                    .map(move |x| {
                        (-offset..=offset).filter_map(move |z| {
                            let nearby = Point3 {
                                x: self.x + x,
                                y: self.y + y,
                                z: self.z + z,
                            };
                            (nearby != *self).then(|| nearby)
                        })
                    })
                    .flatten()
            })
            .flatten()
    }
}

impl Point4 {
    #[inline]
    #[allow(dead_code)]
    pub fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self { x, y, z, w }
    }

    pub fn nearby_points(&self, offset: usize) -> impl Iterator<Item = Point4> + '_ {
        let offset = offset as isize;

        (-offset..=offset)
            .map(move |y| {
                (-offset..=offset)
                    .map(move |x| {
                        (-offset..=offset)
                            .map(move |z| {
                                (-offset..=offset).filter_map(move |w| {
                                    let nearby = Point4 {
                                        x: self.x + x,
                                        y: self.y + y,
                                        z: self.z + z,
                                        w: self.w + w,
                                    };
                                    (nearby != *self).then(|| nearby)
                                })
                            })
                            .flatten()
                    })
                    .flatten()
            })
            .flatten()
    }
}

impl Dimension3 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            active_cubes: HashSet::new(),
        }
    }

    pub fn with_initial_grid(initial_grid: &str) -> Result<Self, String> {
        let mut active_cubes = HashSet::new();

        // parse the initial grid
        for (y, line) in initial_grid
            .split("\n")
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            for (x, column) in line.chars().enumerate() {
                match column {
                    '.' => {}
                    '#' => {
                        active_cubes.insert(Point3 {
                            x: x as isize,
                            y: y as isize,
                            z: 0,
                        });
                    }
                    other => {
                        return Err(format!(
                            "Invalid character while parsing initial grid: {:?}",
                            other
                        ))
                    }
                }
            }
        }

        Ok(Self { active_cubes })
    }

    pub fn perform_cycle(&mut self) {
        let mut new_grid: HashSet<Point3> = HashSet::new();

        for active_cube_pos in self.active_cubes.iter() {
            // There are, in total, 26 nearby cubes in 1-cube distance (9 on top, 8 horizontally, and 9 below). Let's
            // analyze all of them and check which ones are active.
            let nearby_active_count = active_cube_pos.nearby_points(1).fold(0, |prev, nearby| {
                if self.active_cubes.contains(&nearby) {
                    prev + 1
                } else {
                    prev
                }
            });

            // If the nearby count is 2 or 3, we should keep the cube active.
            if nearby_active_count == 2 || nearby_active_count == 3 {
                new_grid.insert(*active_cube_pos);
            }

            for nearby_cube in active_cube_pos.nearby_points(1) {
                if !self.active_cubes.contains(&nearby_cube) {
                    // if we're here, the current nearby cube is an inactive cube. Now we need to check if exactly 3 of
                    // its neighbors are active.
                    let active_count = nearby_cube.nearby_points(1).fold(0, |prev, nearby| {
                        prev + self.active_cubes.contains(&nearby) as u32
                    });

                    if active_count == 3 {
                        new_grid.insert(nearby_cube);
                    }
                }
            }
        }

        std::mem::swap(&mut new_grid, &mut self.active_cubes);
    }

    pub fn active_cubes_amount(&self) -> usize {
        self.active_cubes.len()
    }
}

impl Dimension4 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            active_cubes: HashSet::new(),
        }
    }

    pub fn with_initial_grid(initial_grid: &str) -> Result<Self, String> {
        let mut active_cubes = HashSet::new();

        // parse the initial grid
        for (y, line) in initial_grid
            .split("\n")
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            for (x, column) in line.chars().enumerate() {
                match column {
                    '.' => {}
                    '#' => {
                        active_cubes.insert(Point4 {
                            x: x as isize,
                            y: y as isize,
                            z: 0,
                            w: 0,
                        });
                    }
                    other => {
                        return Err(format!(
                            "Invalid character while parsing initial grid: {:?}",
                            other
                        ))
                    }
                }
            }
        }

        Ok(Self { active_cubes })
    }

    pub fn perform_cycle(&mut self) {
        let mut new_grid: HashSet<Point4> = HashSet::new();

        for active_cube_pos in self.active_cubes.iter() {
            // There are, in total, 26 nearby cubes in 1-cube distance (9 on top, 8 horizontally, and 9 below). Let's
            // analyze all of them and check which ones are active.
            let nearby_active_count = active_cube_pos.nearby_points(1).fold(0, |prev, nearby| {
                if self.active_cubes.contains(&nearby) {
                    prev + 1
                } else {
                    prev
                }
            });

            // If the nearby count is 2 or 3, we should keep the cube active.
            if nearby_active_count == 2 || nearby_active_count == 3 {
                new_grid.insert(*active_cube_pos);
            }

            for nearby_cube in active_cube_pos.nearby_points(1) {
                if !self.active_cubes.contains(&nearby_cube) {
                    // if we're here, the current nearby cube is an inactive cube. Now we need to check if exactly 3 of
                    // its neighbors are active.
                    let active_count = nearby_cube.nearby_points(1).fold(0, |prev, nearby| {
                        prev + self.active_cubes.contains(&nearby) as u32
                    });

                    if active_count == 3 {
                        new_grid.insert(nearby_cube);
                    }
                }
            }
        }

        std::mem::swap(&mut new_grid, &mut self.active_cubes);
    }

    pub fn active_cubes_amount(&self) -> usize {
        self.active_cubes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Point3;

    #[test]
    fn nearby_points() {
        assert_eq!(Point3::new(10, 20, 30).nearby_points(1).count(), 26);
    }
}
