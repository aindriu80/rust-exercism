pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0 || sides[1] == 0 || sides[2] == 0 {
            return None;
        }

        // Check triangle inequality
        if sides[0] + sides[1] <= sides[2]
            || sides[1] + sides[2] <= sides[0]
            || sides[0] + sides[2] <= sides[1]
        {
            return None;
        }

        // Create a new Triangle with the given sides
        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        // A scalene triangle has NO equal sides
        self.sides[0] != self.sides[1]
            && self.sides[1] != self.sides[2]
            && self.sides[0] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        // An isosceles triangle has AT LEAST two equal sides
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }
}
