use crate::actions::{Action, Direction, Orientation, Position};

pub fn part_1(actions: &[Action]) -> i64 {
    let init_state = Ship {
        facing: Orientation::E,
        position: Position { north: 0, east: 0 },
    };

    let end_state_1 = actions.iter().fold(init_state, |current_state, action| {
        take_action(&current_state, action)
    });

    end_state_1.position.north.abs() + end_state_1.position.east.abs()
}

fn take_action(ship: &Ship, action: &Action) -> Ship {
    match action {
        Action::Move(orientation, v) => Ship {
            position: ship.position.move_towards(*orientation, *v),
            ..*ship
        },
        Action::Rotate(direction, v) => Ship {
            facing: (0..*v).fold(ship.facing, |from, _| rotate(from, *direction)),
            ..*ship
        },
        Action::F(v) => take_action(ship, &Action::Move(ship.facing, *v)),
    }
}

fn rotate(from: Orientation, direction: Direction) -> Orientation {
    match direction {
        Direction::L => match from {
            Orientation::N => Orientation::W,
            Orientation::E => Orientation::N,
            Orientation::S => Orientation::E,
            Orientation::W => Orientation::S,
        },
        Direction::R => match from {
            Orientation::N => Orientation::E,
            Orientation::E => Orientation::S,
            Orientation::S => Orientation::W,
            Orientation::W => Orientation::N,
        },
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Ship {
    facing: Orientation,
    position: Position,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_ACTIONS: [Action; 5] = [
        Action::F(10),
        Action::Move(Orientation::N, 3),
        Action::F(7),
        Action::Rotate(Direction::R, 1),
        Action::F(11),
    ];

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_ACTIONS), 25);
    }
}
