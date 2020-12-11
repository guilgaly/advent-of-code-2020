use std::slice::Iter;
use std::str::FromStr;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let waiting_area = INPUT.parse::<WaitingArea>()?;

    let end_state_1 = common::time_execution("Part 1", || {
        find_end_state(&waiting_area, 4, |current_state, x, y, direction| {
            current_state.get_next_pos(x, y, direction)
        })
    });
    println!("Part 1 result: {}", end_state_1.occupied_seats());

    let end_state_2 = common::time_execution("Part 2", || {
        find_end_state(&waiting_area, 5, |current_state, x, y, direction| {
            current_state.get_first_seat_in_direction(x, y, direction)
        })
    });
    println!("Part 2 result: {}", end_state_2.occupied_seats());

    Ok(())
}

type FindNeighbour = for<'a> fn(&'a WaitingArea, usize, usize, &Direction) -> Option<&'a Position>;

fn find_end_state(
    init_state: &WaitingArea,
    max_neighbours: usize,
    find_neigbour: FindNeighbour,
) -> WaitingArea {
    let mut current_state = init_state.clone();
    loop {
        let next = next_state(&current_state, max_neighbours, find_neigbour);
        if next == current_state {
            return next;
        } else {
            current_state = next;
        }
    }
}

fn next_state(
    current_state: &WaitingArea,
    max_neighbours: usize,
    find_neigbour: FindNeighbour,
) -> WaitingArea {
    let mut new_rows = Vec::with_capacity(current_state.height as usize);

    for j in 0..current_state.height {
        let mut row: Vec<Position> = Vec::with_capacity(current_state.width as usize);

        for i in 0..current_state.width {
            let count_neighbours = || {
                Direction::all().fold(0, |acc, direction| {
                    match find_neigbour(current_state, i, j, direction) {
                        Some(Position::OccupiedSeat) => acc + 1,
                        _ => acc,
                    }
                })
            };

            let new_pos = match current_state.rows[j as usize][i as usize] {
                Position::Floor => Position::Floor,
                Position::EmptySeat => {
                    if count_neighbours() == 0 {
                        Position::OccupiedSeat
                    } else {
                        Position::EmptySeat
                    }
                }
                Position::OccupiedSeat => {
                    if count_neighbours() >= max_neighbours {
                        Position::EmptySeat
                    } else {
                        Position::OccupiedSeat
                    }
                }
            };

            row.push(new_pos);
        }

        new_rows.push(row);
    }

    WaitingArea {
        rows: new_rows,
        width: current_state.width,
        height: current_state.height,
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Direction(i32, i32);

impl Direction {
    fn all() -> Iter<'static, Direction> {
        [
            Direction(-1, -1),
            Direction(-1, 0),
            Direction(-1, 1),
            Direction(0, -1),
            Direction(0, 1),
            Direction(1, -1),
            Direction(1, 0),
            Direction(1, 1),
        ]
        .iter()
    }

    fn shift_within(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        let calc_x = (x as i32) + self.0;
        let calc_y = (y as i32) + self.1;
        if calc_x < 0 || calc_y < 0 {
            None
        } else {
            let x = calc_x as usize;
            let y = calc_y as usize;
            if x >= width || y >= height {
                None
            } else {
                Some((x, y))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct WaitingArea {
    rows: Vec<Vec<Position>>,
    width: usize,
    height: usize,
}

impl WaitingArea {
    fn get_next_pos(&self, x: usize, y: usize, direction: &Direction) -> Option<&Position> {
        direction
            .shift_within(x, y, self.width, self.height)
            .map(|(x, y)| &self.rows[y][x])
    }

    fn get_first_seat_in_direction(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Option<&Position> {
        direction
            .shift_within(x, y, self.width, self.height)
            .and_then(|(x, y)| match &self.rows[y][x] {
                Position::Floor => self.get_first_seat_in_direction(x, y, direction),
                other => Some(other),
            })
    }

    fn occupied_seats(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .filter(|pos| **pos == Position::OccupiedSeat)
            .count()
    }
}

impl FromStr for WaitingArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => Ok(Position::Floor),
                        'L' => Ok(Position::EmptySeat),
                        '#' => Ok(Position::OccupiedSeat),
                        _ => Err(format!("Not a valid position: {}", s)),
                    })
                    .collect::<Result<Vec<Position>, String>>()
            })
            .collect::<Result<Vec<Vec<Position>>, String>>()?;
        let init_width = rows
            .get(0)
            .map(|row| row.len())
            .ok_or("Pattern must contain at least one line")?;
        let height = rows.len();

        if init_width == 0 {
            Err("Width must be > 0".to_owned())
        } else if height == 0 {
            Err("Height must be > 0".to_owned())
        } else {
            let width = rows.iter().try_fold(init_width, |width, line| {
                if line.len() != width {
                    Err(format!("All rows must have a width of {}", width))
                } else {
                    Ok(width)
                }
            })?;

            Ok(WaitingArea {
                rows,
                width,
                height,
            })
        }
    }
}
