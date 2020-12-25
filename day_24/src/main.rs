use common::itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("input");
static TEST_INPUT: &str = include_str!("test_input");

fn main() -> Result<(), String> {
    let instructions: Vec<Vec<Direction>> =
        INPUT.lines().map(|line| parse_instruction(line)).collect();

    let coords = instructions
        .iter()
        .map(|instr| reduce_instruction(instr))
        .map(|c| (c.clone(), c))
        .into_group_map();

    let black_squares: HashSet<Tile> = coords
        .into_iter()
        .filter_map(|(coord, list)| {
            if list.len() % 2 == 0 {
                None
            } else {
                Some(coord)
            }
        })
        .collect();
    println!("Part 1 result: {}", black_squares.len());

    let init_state = GameState::new(black_squares);
    let final_state = (0..100).fold(init_state, |prev, _| next_cycle(&prev));
    println!("Part 2 result: {}", final_state.blacks.len());

    Ok(())
}

fn next_cycle(prev: &GameState) -> GameState {
    let mut blacks: HashSet<Tile> = HashSet::new();

    for q in (prev.q_min - 1)..=(prev.q_max + 1) {
        for r in (prev.r_min - 1)..=(prev.r_max + 1) {
            let tile = Tile { q, r };
            let neighbours = count_neighbours(&tile, &prev.blacks);
            let is_black = prev.blacks.contains(&tile);
            if (is_black && neighbours > 0 && neighbours <= 2) || (!is_black && neighbours == 2) {
                blacks.insert(tile);
            }
        }
    }

    GameState::new(blacks)
}

fn count_neighbours(tile: &Tile, blacks: &HashSet<Tile>) -> usize {
    let mut count = 0;
    let neighbours = [
        Tile {
            q: tile.q + 1,
            r: tile.r,
        },
        Tile {
            q: tile.q,
            r: tile.r + 1,
        },
        Tile {
            q: tile.q - 1,
            r: tile.r + 1,
        },
        Tile {
            q: tile.q - 1,
            r: tile.r,
        },
        Tile {
            q: tile.q,
            r: tile.r - 1,
        },
        Tile {
            q: tile.q + 1,
            r: tile.r - 1,
        },
    ];
    for neighbour in &neighbours {
        if blacks.contains(neighbour) {
            count += 1;
        }
    }
    count
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct GameState {
    blacks: HashSet<Tile>,
    q_min: i64,
    q_max: i64,
    r_min: i64,
    r_max: i64,
}

impl GameState {
    fn new(blacks: HashSet<Tile>) -> GameState {
        let q_min = blacks.iter().map(|c| c.q).min().unwrap_or(0);
        let q_max = blacks.iter().map(|c| c.q).max().unwrap_or(0);
        let r_min = blacks.iter().map(|c| c.r).min().unwrap_or(0);
        let r_max = blacks.iter().map(|c| c.r).max().unwrap_or(0);

        GameState {
            blacks,
            q_min,
            q_max,
            r_min,
            r_max,
        }
    }
}

fn parse_instruction(instr: &str) -> Vec<Direction> {
    let mut instruction = Vec::new();
    instr
        .chars()
        .fold(None, |prev_char, char| match (prev_char, char) {
            (Some('s'), 'e') => {
                instruction.push(Direction::SE);
                None
            }
            (Some('s'), 'w') => {
                instruction.push(Direction::SW);
                None
            }
            (Some('n'), 'e') => {
                instruction.push(Direction::NE);
                None
            }
            (Some('n'), 'w') => {
                instruction.push(Direction::NW);
                None
            }
            (None, 'e') => {
                instruction.push(Direction::E);
                None
            }
            (None, 'w') => {
                instruction.push(Direction::W);
                None
            }
            (None, 's') => Some('s'),
            (None, 'n') => Some('n'),
            _ => panic!(),
        });
    instruction
}

fn reduce_instruction(instruction: &Vec<Direction>) -> Tile {
    let mut q = 0;
    let mut r = 0;

    for direction in instruction {
        match direction {
            Direction::E => {
                q += 1;
            }
            Direction::SE => {
                r += 1;
            }
            Direction::SW => {
                q -= 1;
                r += 1;
            }
            Direction::W => {
                q -= 1;
            }
            Direction::NW => {
                r -= 1;
            }
            Direction::NE => {
                q += 1;
                r -= 1;
            }
        }
    }

    Tile { q, r }
}

/// https://www.redblobgames.com/grids/hexagons/
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Tile {
    q: i64,
    r: i64,
}
//
// impl PartialEq for AxialCoordinate {
//     fn eq(&self, other: &Self) -> bool {
//         let distance = ((self.q - other.q).abs()
//             + (self.q + self.r - other.q - other.r).abs()
//             + (self.r - other.r).abs())
//             / 2;
//         distance == 0
//     }
// }
//
// impl Eq for AxialCoordinate {}

enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
