use common::itertools::Itertools;
use common::lazy_static::lazy_static;
use common::regex::Regex;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

static INPUT: &str = include_str!("input");
static TEST_INPUT: &str = include_str!("test_input");

const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

type BorderMap = HashMap<String, Vec<usize>>;
type TilesMap = HashMap<usize, Tile>;

fn main() -> Result<(), String> {
    let tiles = parse_tiles(INPUT)?;
    let mut border_map = HashMap::new();
    for tile in tiles.values() {
        let id = tile.id;
        for edge in &tile.get_edges() {
            border_map.entry(edge.to_string()).or_insert(Vec::new()).push(id);
            border_map.entry(edge.chars().rev().collect()).or_insert(Vec::new()).push(id);
        }
    }

    let res_1 = part_one(&border_map);
    println!("Part 1 result: {}", res_1);

    let res_2 = part_two(&tiles, &border_map, 3181);
    println!("Part 2 result: {}", res_2);

    Ok(())
}

fn part_one(border_map: &BorderMap) -> usize {
    let mut count_map = HashMap::new();
    for ids in border_map.values().filter(|ids| ids.len() == 1) {
        *count_map.entry(ids[0]).or_insert(0) += 1;
    }
    count_map.iter()
        .filter(|&(_, &c)| c == 4)
        .map(|(id,_)| {
            println!("Corner: {}", id);
            id
        })
        .product()
}

fn part_two(tiles: &TilesMap, border_map: &BorderMap, corner: usize) -> usize {
    let monster_coords = MONSTER.iter()
        .enumerate()
        .flat_map(|(i,row)| row.chars()
            .enumerate()
            .filter(|&(_,c)| c == '#')
            .map(move |(j,_)| (i as isize - 1, j as isize))
        )
        .collect::<HashSet<_>>();
    let mut image = build_image(tiles, border_map, corner);
    let total = image.iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count();
    loop {
        match find_monsters(&image, &monster_coords) {
            0 => image = rotate(&image),
            m => return total - m * monster_coords.len(),
        }
    }
}

fn parse_tiles(input: &str) -> Result<TilesMap, String> {
    input
        .split("\n\n")
        .map(|part| part.parse::<Tile>().map(|tile| (tile.id, tile)))
        .collect()
}

#[derive(Clone, Debug, Default)]
struct Tile {
    v: Vec<Vec<char>>,
    id: usize,
}

impl Tile {

    fn get_edges(&self) -> [String; 4] {
        let top = self.v[0].iter().collect::<String>();
        let bottom = self.v[9].iter().collect::<String>();
        let (mut left, mut right) = (String::new(), String::new());
        for i in 0..10 {
            left.push(self.v[i][0]);
            right.push(self.v[i][9]);
        }
        [top, bottom, left, right]
    }

    fn get_neighbour(&self, border_map: &BorderMap, n: usize) -> Option<usize> {
        let matches = match n {
            0 => &border_map[&self.get_edges()[0]],
            1 => &border_map[&self.get_edges()[3]],
            2 => &border_map[&self.get_edges()[1]],
            3 => &border_map[&self.get_edges()[2]],
            _ => unreachable!()
        };
        matches.iter().find(|&&id| id != self.id).copied()
    }

    fn rotate(&mut self) { self.v = rotate(&self.v) }

    fn match_right(&self, border_map: &BorderMap, tiles: &TilesMap) -> Self {
        let id = self.get_neighbour(border_map, 1).unwrap();
        let mut tile = tiles[&id].clone();

        // rotate it to the correct position
        while tile.get_neighbour(border_map, 3) != Some(self.id) { tile.rotate() }

        // if the edges match but aren't equal it must be flipped!
        if (0..10).any(|i| self.v[i][9] != tile.v[i][0]) {
            for i in 0..5 { tile.v.swap(i, 9 - i) }
        }
        tile
    }

    fn match_down(&self, border_map: &BorderMap, tiles: &TilesMap) -> Self {
        let id = self.get_neighbour(border_map, 2).unwrap();
        let mut tile = tiles[&id].clone();

        // rotate it to the correct position
        while tile.get_neighbour(border_map, 0) != Some(self.id) { tile.rotate() }

        // if the edges match but aren't equal it must be flipped!
        if self.v[9] != tile.v[0] {
            for s in &mut tile.v { s.reverse() }
        }
        tile
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = self
            .v
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        f.write_fmt(format_args!("Tile {}:\n{}", self.id, str))
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
            .map(|cap| cap.get(1).unwrap().as_str().parse::<usize>().unwrap())
            .unwrap();

        let v: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect();
        Ok(Tile { id, v })
    }
}

fn rotate(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let (h,w) = (v.len(), v[0].len());
    let mut rot = vec![vec!['\0'; w]; h];
    for (i,j) in (0..h).cartesian_product(0..w) {
        rot[j][w-1-i] = v[i][j];
    }
    rot
}


fn build_image(tiles: &TilesMap, border_map: &BorderMap, corner: usize) -> Vec<Vec<char>> {
    // let images = IMAGES.iter().copied().collect::<HashMap<_,_>>();

    // align the corner to fit in the top-left
    let mut starting_corner = tiles[&corner].clone();
    loop {
        let n1 = starting_corner.get_neighbour(border_map, 0);
        let n2 = starting_corner.get_neighbour(border_map, 3);
        match (n1,n2) {
            (None, None) => break,
            _ => starting_corner.rotate(),
        }
    }

    let mut image = vec![vec![Tile::default(); 12]; 12];
    image[0][0] = starting_corner;
    // match the first tile in each row to the one above
    for i in 1..12 {
        image[i][0] = image[i-1][0].match_down(border_map, tiles);
    }
    // for tile, match to the previous tile in the row
    for (i,j) in (0..12).cartesian_product(1..12) {
        image[i][j] = image[i][j-1].match_right(border_map, tiles);
    }

    // tiles are placed and rotated correctly, now build the actual image
    let mut actual_image = vec![Vec::new(); 8 * 12];
    for (i,j) in (0..12).cartesian_product(0..12) {
        let tile = &image[i][j];
        for k in 1..9 {
            actual_image[i * 8 + (k-1)].extend(&tile.v[k][1..9]);
        }
    }
    actual_image
}

fn find_monsters(image: &[Vec<char>], monster_coords: &HashSet<(isize,isize)>) -> usize {
    let positions = image.iter()
        .enumerate()
        .flat_map(|(i,row)| row.iter()
            .enumerate()
            .filter(|&(_,&c)| c == '#')
            .map(move |(j,_)| (i as isize, j as isize))
        )
        .collect::<HashSet<_>>();
    positions.iter()
        .filter(|(i,j)| monster_coords.iter()
            .map(|(a,b)| (i+a,j+b))
            .all(|pos| positions.contains(&pos))
        )
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
