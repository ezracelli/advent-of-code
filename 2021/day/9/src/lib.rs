mod coordinate;

use std::collections::BTreeSet;
use std::convert::Infallible;
use std::ops::{Deref, Index};
use std::str::FromStr;

use crate::coordinate::Coordinate;

#[derive(Debug)]
pub struct Input(Vec<Vec<usize>>);

impl Input {
    fn basin(&self, basin: &mut BTreeSet<Coordinate>, coordinate: Coordinate) {
        let depth = self[coordinate];

        if depth == 9 {
            return;
        }

        basin.insert(coordinate);

        if coordinate.y != 0 {
            let up = Coordinate::new(coordinate.x, coordinate.y - 1);

            if !basin.contains(&up) && self[up] > depth {
                self.basin(basin, up);
            }
        }

        if coordinate.y != self.len() - 1 {
            let down = Coordinate::new(coordinate.x, coordinate.y + 1);

            if !basin.contains(&down) && self[down] > depth {
                self.basin(basin, down);
            }
        }

        if coordinate.x != 0 {
            let left = Coordinate::new(coordinate.x - 1, coordinate.y);

            if !basin.contains(&left) && self[left] > depth {
                self.basin(basin, left);
            }
        }

        if coordinate.x != self[coordinate.y].len() - 1 {
            let right = Coordinate::new(coordinate.x + 1, coordinate.y);

            if !basin.contains(&right) && self[right] > depth {
                self.basin(basin, right);
            }
        }
    }

    fn basins(&self) -> BTreeSet<BTreeSet<Coordinate>> {
        let mut basins = BTreeSet::new();

        for coordinate in self.low_points() {
            let mut basin = BTreeSet::new();
            self.basin(&mut basin, coordinate);

            basins.insert(basin);
        }

        basins
    }

    fn low_points(&self) -> BTreeSet<Coordinate> {
        let mut low_points = BTreeSet::new();

        for y in 0..self.len() {
            for x in 0..self[y].len() {
                let coordinate = Coordinate::new(x, y);
                let depth = self[coordinate];

                let up = if y == 0 {
                    usize::MAX
                } else {
                    self[Coordinate::new(x, y - 1)]
                };

                let down = if y == self.len() - 1 {
                    usize::MAX
                } else {
                    self[Coordinate::new(x, y + 1)]
                };

                let left = if x == 0 {
                    usize::MAX
                } else {
                    self[Coordinate::new(x - 1, y)]
                };

                let right = if x == self[y].len() - 1 {
                    usize::MAX
                } else {
                    self[Coordinate::new(x + 1, y)]
                };

                if depth < up && depth < down && depth < left && depth < right {
                    low_points.insert(coordinate);
                }
            }
        }

        low_points
    }
}

impl Deref for Input {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<usize> for Input {
    type Output = Vec<usize>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Coordinate> for Input {
    type Output = usize;

    fn index(&self, coordinate: Coordinate) -> &Self::Output {
        &self.0[coordinate.y][coordinate.x]
    }
}

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        self.low_points()
            .into_iter()
            .map(|coordinate| self[coordinate] + 1)
            .sum()
    }

    fn run_part2(&self) -> Self::Answer {
        let mut basins = self.basins().into_iter().collect::<Vec<_>>();
        basins.sort_by_key(|basin| basin.len());

        basins
            .into_iter()
            .rev()
            .take(3)
            .map(|basin| basin.len())
            .product()
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            value
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|char| char.to_string().parse().unwrap())
                        .collect()
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_day_9() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_9_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 15;
        assert_eq!(parse_day_9().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_9_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 1134;
        assert_eq!(parse_day_9().run_part2(), ANSWER);
    }
}
