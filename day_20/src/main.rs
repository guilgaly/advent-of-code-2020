use common::itertools::Itertools;
use common::lazy_static::lazy_static;
use common::regex::Regex;

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

static INPUT: &str = include_str!("input");
static TEST_INPUT: &str = include_str!("test_input");

fn main() -> Result<(), String> {
    let tiles_by_id = parse_tiles(TEST_INPUT)?;

    let top_left = tiles_by_id.iter().find(|(tile_id, tile_variants)| {
        tile_variants.iter().find(|tile| {
            let other_variants = tiles_by_id.iter()
                .filter(|(i, _)| i != tile_id)
                .flat_map(|(_, t)| t)
                .collect::<Vec<_>>();
            !other_variants.iter().any(|other| other.bottom_border() == tile.top_border()) &&
                !other_variants.iter().any(|other| other.right_border() == tile.left_border()) &&
                other_variants.iter().any(|other| other.top_border() == tile.bottom_border()) &&
                other_variants.iter().any(|other| other.left_border() == tile.right_border())

        }).is_some()
    });
    println!("{:?}", top_left);

    Ok(())
}

fn parse_tiles(input: &str) -> Result<HashMap<u64, Vec<Tile>>, String> {
    input
        .split("\n\n")
        .map(|part| {
            part.parse::<Tile>().map(|tile| {
                let tiles = vec![
                    tile.clone(),
                    tile.rotate(),
                    tile.rotate().rotate(),
                    tile.rotate().rotate(),
                    tile.flip(),
                    tile.flip().rotate(),
                    tile.flip().rotate().rotate(),
                    tile.flip().rotate().rotate().rotate(),
                ];
                (tile.id, tiles)
            })
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Tile {
    id: u64,
    pixels: [[bool; 10]; 10],
}

impl Tile {
    fn flip(&self) -> Tile {
        let mut pixels = [[false; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                pixels[x][y] = self.pixels[9 - x][y];
            }
        }
        Tile {
            id: self.id,
            pixels,
        }
    }

    fn rotate(&self) -> Tile {
        let mut pixels = [[false; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                pixels[x][y] = self.pixels[y][9 - x];
            }
        }
        Tile {
            id: self.id,
            pixels,
        }
    }

    fn top_border(&self) -> [bool; 10] {
        self.pixels[0]
    }

    fn bottom_border(&self) -> [bool; 10] {
        self.pixels[9]
    }

    fn left_border(&self) -> [bool; 10] {
        let mut border = [false; 10];
        for x in 0..10 {
            border[x] = self.pixels[x][0];
        }
        border
    }

    fn right_border(&self) -> [bool; 10] {
        let mut border = [false; 10];
        for x in 0..10 {
            border[x] = self.pixels[x][9];
        }
        border
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = self
            .pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|p| if *p { '#' } else { '.' })
                    .collect::<String>()
            })
            .join("\n");
        f.write_str(&str)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^Tile ([0-9]+):$").unwrap();
        }
        // let REGEX = Regex::new(r"^Tile ([0-9]+):$").unwrap();
        let id = REGEX
            .captures(lines.next().unwrap())
            .map(|cap| cap.get(1).unwrap().as_str().parse::<u64>().unwrap())
            .unwrap();

        let mut pixels = [[false; 10]; 10];
        lines.enumerate().for_each(|(x, line)| {
            line.chars().enumerate().for_each(|(y, c)| {
                if c == '#' {
                    pixels[x][y] = true;
                }
            })
        });
        Ok(Tile { id, pixels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
