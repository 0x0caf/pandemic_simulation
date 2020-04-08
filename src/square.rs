use rgx::math::*;

#[derive(Clone)]
pub struct Square {
    pub center: Vector2<f32>,
    pub top_right: Vector2<f32>,
    pub bottom_left: Vector2<f32>,
    size: f32,
    half_size: f32,
}

impl Square {
    pub fn new(center: Vector2<f32>, size: f32) -> Square {
        let half_size = size * 0.5;
        Square {
            center,
            bottom_left: Vector2::new(center.x - half_size, center.y - half_size),
            top_right: Vector2::new(center.x + half_size, center.y + half_size),
            size,
            half_size,
        }
    }

    pub fn update(&mut self, center: &Vector2<f32>) {
        self.center.x = center.x;
        self.center.y = center.y;
        self.top_right.x = center.x + self.half_size;
        self.top_right.y = center.y + self.half_size;
        self.bottom_left.x = center.x - self.half_size;
        self.bottom_left.y = center.y - self.half_size;
    }

    pub fn add_half_size_bias(&self) -> Square {
        Square::new(self.center.clone(), self.size + self.size)
    }

    pub fn intersects(&self, other: &Square) -> bool {
        let x_intersect = (other.bottom_left.x >= self.bottom_left.x
            && other.bottom_left.x <= self.top_right.x)
            || (other.top_right.x >= self.bottom_left.x && other.top_right.x <= self.top_right.x);

        x_intersect
            && ((other.bottom_left.y >= self.bottom_left.y
                && other.bottom_left.y <= self.top_right.y)
                || (other.top_right.y >= self.bottom_left.y
                    && other.top_right.y <= self.top_right.y))
    }
}
