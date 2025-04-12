pub struct Triangle {
    sides: [u64; 3],
}
enum TriangleType {}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0 || sides[1] == 0 || sides[2] == 0 {
            return None;
        }

        if sides[0] == sides[1] && sides[1] == sides[2] && sides[0] == sides[2] {
            Some(Triangle {}) // Return Some with a Triangle instance
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        todo!("this");
    }

    pub fn is_scalene(&self) -> bool {
        todo!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        todo!("Determine if the Triangle is isosceles.");
    }
}
