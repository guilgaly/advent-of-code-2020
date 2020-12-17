use std::collections::HashSet;
use std::str::FromStr;

pub fn part_2(input: &str) -> Result<usize, String> {
    let init_state = input.parse::<GameState>()?;
    let final_state = (0..6).fold(init_state, |prev, _| next_cycle(&prev));
    Ok(final_state.actives.len())
}

fn next_cycle(prev: &GameState) -> GameState {
    let mut actives: HashSet<Cube> = HashSet::new();

    for x in (prev.x_min - 1)..=(prev.x_max + 1) {
        for y in (prev.y_min - 1)..=(prev.y_max + 1) {
            for z in (prev.z_min - 1)..=(prev.z_max + 1) {
                for w in (prev.w_min - 1)..=(prev.w_max + 1) {
                    let cube = Cube { x, y, z, w };
                    let neighbours = count_neighbours(&cube, &prev.actives);
                    if neighbours == 3 || (prev.actives.contains(&cube) && neighbours == 2) {
                        actives.insert(cube);
                    }
                }
            }
        }
    }

    let x_min = actives.iter().map(|c| c.x).min().unwrap_or(0);
    let x_max = actives.iter().map(|c| c.x).max().unwrap_or(0);
    let y_min = actives.iter().map(|c| c.y).min().unwrap_or(0);
    let y_max = actives.iter().map(|c| c.y).max().unwrap_or(0);
    let z_min = actives.iter().map(|c| c.z).min().unwrap_or(0);
    let z_max = actives.iter().map(|c| c.z).max().unwrap_or(0);
    let w_min = actives.iter().map(|c| c.w).min().unwrap_or(0);
    let w_max = actives.iter().map(|c| c.w).max().unwrap_or(0);

    GameState {
        actives,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
        w_min,
        w_max,
    }
}

fn count_neighbours(cube: &Cube, actives: &HashSet<Cube>) -> usize {
    let mut count = 0;
    for x in (cube.x - 1)..=(cube.x + 1) {
        for y in (cube.y - 1)..=(cube.y + 1) {
            for z in (cube.z - 1)..=(cube.z + 1) {
                for w in (cube.w - 1)..=(cube.w + 1) {
                    if (x != cube.x || y != cube.y || z != cube.z || w != cube.w)
                        && actives.contains(&Cube { x, y, z, w })
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct GameState {
    actives: HashSet<Cube>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
    w_min: i64,
    w_max: i64,
}

impl FromStr for GameState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let actives: HashSet<Cube> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Cube {
                            x: x as i64,
                            y: y as i64,
                            z: 0,
                            w: 0,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect();
        let x_max = actives.iter().map(|c| c.x).max().unwrap();
        let y_max = actives.iter().map(|c| c.y).max().unwrap();

        Ok(GameState {
            actives,
            x_min: 0,
            x_max,
            y_min: 0,
            y_max,
            z_min: 0,
            z_max: 0,
            w_min: 0,
            w_max: 0,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> Result<(), String> {
        let input = ".#.
..#
###";
        assert_eq!(part_2(input)?, 848);
        Ok(())
    }
}
