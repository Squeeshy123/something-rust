pub mod Vec2;

struct Vec2 {
    x: i32,
    y: i32,
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}
