use crate::actions::{Action, Direction, Position};

pub fn part_2(actions: &[Action]) -> i64 {
    let init_positions = Positions {
        ship: Position { north: 0, east: 0 },
        waypoint: Position { north: 1, east: 10 },
    };

    let end_positions = actions.iter().fold(init_positions, |current_state, action| {
        take_action(&current_state, action)
    });

    end_positions.ship.north.abs() + end_positions.ship.east.abs()
}

fn take_action(positions: &Positions, action: &Action) -> Positions {
    match action {
        Action::Move(orientation, v) => Positions {
            waypoint: positions.waypoint.move_towards(*orientation, *v),
            ..*positions
        },
        Action::Rotate(direction, v) => Positions {
            waypoint: (0..*v).fold(positions.waypoint, |from, _| rotate(from, *direction)),
            ..*positions
        },
        Action::F(v) => Positions {
            ship: positions.ship.move_by(positions.waypoint.scaled(*v)),
            ..*positions
        },
    }
}

fn rotate(from: Position, direction: Direction) -> Position {
    let (north, east) = match direction {
        Direction::L => (from.east, -from.north),
        Direction::R => (-from.east, from.north),
    };
    Position { north, east }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Positions {
    ship: Position,
    waypoint: Position,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::Orientation;

    static TEST_ACTIONS: [Action; 5] = [
        Action::F(10),
        Action::Move(Orientation::N, 3),
        Action::F(7),
        Action::Rotate(Direction::R, 1),
        Action::F(11),
    ];

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_ACTIONS), 286);
    }
}
