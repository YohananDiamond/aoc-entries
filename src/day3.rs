mod aoc;

fn main() {
    aoc::start("day3.txt", part1, part2);
}

struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Tile {
    Tree,
    Nothing,
}

#[derive(Debug)]
struct Map {
    inner: Vec<Vec<Tile>>,
    line_len: usize,
}

impl Map {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut result = Vec::new();
        let mut iter = input.split("\n").filter(|s| !s.is_empty());

        let accepted_len = {
            let first_line = iter.next().ok_or_else(|| format!("Input is empty"))?;
            result.push(Self::parse_line(first_line)?);

            first_line.len()
        };

        for line in iter {
            if line.len() != accepted_len {
                return Err(format!(
                    "Line length ({}) is different from first line length ({})",
                    line.len(),
                    accepted_len
                ));
            }

            result.push(Self::parse_line(line)?);
        }

        Ok(Self {
            inner: result,
            line_len: accepted_len,
        })
    }

    #[inline]
    pub fn line_len(&self) -> usize {
        self.line_len
    }

    fn parse_line(line: &str) -> Result<Vec<Tile>, String> {
        line.chars()
            .map(|c| match c {
                '.' => Ok(Tile::Nothing),
                '#' => Ok(Tile::Tree),
                _ => Err(format!("Unknown character: {:?}", c)),
            })
            .collect()
    }

    pub fn tree_collisions_with_slope(&self, slope: Point2<usize>) -> usize {
        let mut collisions: usize = 0;
        let mut line: usize = 0;
        let mut column: usize = 0;

        loop {
            {
                let Point2 { x, y } = slope;
                line += y;
                column += x;
            }

            // wrap line
            column = column % self.line_len;

            // stop if it has been through the bottom of the screen already
            if line >= self.inner.len() {
                break collisions;
            }

            // check collision
            if let Tile::Tree = self.inner[line][column] {
                collisions += 1;
            }
        }
    }
}

fn part1(input: &str) -> Result<String, String> {
    let map = Map::new(input).map_err(|e| format!("While parsing input: {}", e))?;

    Ok(format!(
        "{}",
        map.tree_collisions_with_slope(Point2::new(3, 1))
    ))
}

fn part2(input: &str) -> Result<String, String> {
    let map = Map::new(input).map_err(|e| format!("While parsing input: {}", e))?;

    let calc1 = map.tree_collisions_with_slope(Point2::new(1, 1));
    let calc2 = map.tree_collisions_with_slope(Point2::new(3, 1));
    let calc3 = map.tree_collisions_with_slope(Point2::new(5, 1));
    let calc4 = map.tree_collisions_with_slope(Point2::new(7, 1));
    let calc5 = map.tree_collisions_with_slope(Point2::new(1, 2));

    Ok(format!("{}", calc1 * calc2 * calc3 * calc4 * calc5))
}
