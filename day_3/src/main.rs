static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let map = parse_map(INPUT)?;
    println!(
        "Pattern width: {}, height: {}",
        map.pattern_width, map.pattern_height
    );

    let part_1_result = count_trees(&map, (3, 1));
    println!("Part 1 result: {}", part_1_result);

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part_2_result: usize = slopes
        .iter()
        .map(|slope| count_trees(&map, *slope))
        .product();
    println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn parse_map(input: &str) -> Result<Map, String> {
    let pattern = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Ok(Square::Empty),
                    '#' => Ok(Square::Tree),
                    _ => Err(format!("Illegal character {}", char)),
                })
                .collect::<Result<Vec<Square>, String>>()
        })
        .collect::<Result<Vec<Vec<Square>>, String>>()?;

    let init_width = pattern
        .get(0)
        .map(|line| line.len())
        .ok_or("Pattern must contain at least one line")?;
    let pattern_height = pattern.len();

    if init_width == 0 {
        Err("Width must be > 0".to_owned())
    } else if pattern_height == 0 {
        Err("Height must be > 0".to_owned())
    } else {
        let pattern_width = pattern.iter().try_fold(init_width, |width, line| {
            if line.len() != width {
                Err(format!("All lines must have a width of {}", width))
            } else {
                Ok(width)
            }
        })?;

        Ok(Map {
            pattern,
            pattern_width,
            pattern_height,
        })
    }
}

fn count_trees(map: &Map, trajectory: (usize, usize)) -> usize {
    let mut count: usize = 0;
    let mut current_pos: (usize, usize) = (0, 0);

    while let Some(square) = map.square(current_pos) {
        if *square == Square::Tree {
            count += 1;
        }
        current_pos = (current_pos.0 + trajectory.0, current_pos.1 + trajectory.1);
    }

    count
}

#[derive(PartialEq)]
enum Square {
    Empty,
    Tree,
}

/// `pattern` is a collection of lines from top to bottom. Each line is a
/// collection of squares from left to right.
/// The coordinates system, starting at (0, 0), is (x, y) where x goes from left
/// to right, y goes from top to bottom.
struct Map {
    pattern: Vec<Vec<Square>>,
    pattern_width: usize,
    pattern_height: usize,
}

impl Map {
    fn square(&self, (x, y): (usize, usize)) -> Option<&Square> {
        let effective_x = x % self.pattern_width;
        self.pattern
            .get(y)
            .map(|line| line.get(effective_x).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_single_slope() -> Result<(), String> {
        let map = parse_map(TEST_INPUT)?;
        assert_eq!(count_trees(&map, (3, 1)), 7);
        Ok(())
    }

    #[test]
    fn test_all_slopes() -> Result<(), String> {
        let map = parse_map(TEST_INPUT)?;
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let result: usize = slopes
            .iter()
            .map(|slope| count_trees(&map, *slope))
            .product();
        assert_eq!(result, 336);
        Ok(())
    }
}
