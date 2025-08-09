use macroquad::prelude::*;

pub struct Triangle {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
}

#[derive(Clone)]
pub struct Boid {
    pub pos: Vec2,
    pub head: Vec2,
    pub back_left: Vec2,
    pub back_right: Vec2,
    pub speed: f32,
}

impl Boid {
    fn shape(&self) -> Triangle {
        let Boid { pos, head: front, back_left, back_right, speed } = self.clone();
        let v1 = pos + front;
        let v2 = pos + back_left;
        let v3 = pos + back_right;

        Triangle {v1, v2, v3}
    }
}