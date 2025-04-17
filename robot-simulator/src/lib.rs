// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Option<Direction>,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            x,
            y,
            direction: Some(d),
        }
        // todo!()
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            Some(Direction::North) => Direction::East,
            Some(Direction::East) => Direction::South,
            Some(Direction::South) => Direction::West,
            Some(Direction::West) => Direction::North,
            None => unreachable!(),
        };

        Self {
            x: self.x,
            y: self.y,
            direction: Some(new_direction),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction = match self.direction {
            Some(Direction::North) => Direction::West,
            Some(Direction::East) => Direction::North,
            Some(Direction::South) => Direction::East,
            Some(Direction::West) => Direction::South,
            None => unreachable!(),
        };

        Self {
            x: self.x,
            y: self.y,
            direction: Some(new_direction),
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Some(Direction::North) => Self {
                x: self.x,
                y: self.y + 1,
                direction: self.direction,
            },
            Some(Direction::South) => Self {
                x: self.x,
                y: self.y - 1,
                direction: self.direction,
            },
            Some(Direction::East) => Self {
                x: self.x + 1,
                y: self.y,
                direction: self.direction,
            },
            Some(Direction::West) => Self {
                x: self.x - 1,
                y: self.y,
                direction: self.direction,
            },
            None => unreachable!(),
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;

        for c in instructions.chars() {
            robot = match c {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot, // ignorning other characters
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        if self.x == 0 && self.y == 0 {
            (0, 0)
        } else {
            (self.x, self.y)
        }
    }

    pub fn direction(&self) -> &Direction {
        self.direction.as_ref().unwrap()
    }
}
