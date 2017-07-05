use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: u8,
    pub y: u8
}
impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        assert!(x < 5 && y < 5);
        Point {x, y}
    }
    pub fn can_add(&self, s: Step) -> bool {
        let new_x = (self.x as i8) + s.dx;
        let new_y = (self.y as i8) + s.dy;
        new_x >= 0 && new_y >= 0 && new_x < 5 && new_y < 5
    }
}

impl Add<Step> for Point {
    type Output = Point;
    fn add(self, s: Step) -> Point {
        assert!(self.can_add(s));
        Point::new(
            ((self.x as i8) + s.dx) as u8,
            ((self.y as i8) + s.dy) as u8
        )
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Step {
    pub dx: i8,
    pub dy: i8
}
impl Step {
    pub fn new(dx: i8, dy: i8) -> Step {
        assert!(dx > -4 && dy > -4 && dx < 5 && dy < 5 && (dx != 0 || dy != 0));
        Step {dx, dy}
    }

    pub fn flip(&self) -> Step {
        Step::new(-self.dx, -self.dy)
    }
}
