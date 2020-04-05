use rgx::math::*;
// implements the collision box as implemented through rgx

pub struct WindowBox {
    top_left: Vector2<f32>,
    top_right: Vector2<f32>,
    bottom_left: Vector2<f32>,
    bottom_right: Vector2<f32>,
}

pub struct CollisionResult {
    pub direction: Vector2<f32>,
    pub position: Vector2<f32>,
}

impl WindowBox {
    pub fn new(width: i32, height: i32) -> WindowBox {
        let f_width = width as f32;
        let f_height = height as f32;
        WindowBox {
            top_left: Vector2::new(0.0, f_height),
            top_right: Vector2::new(f_width, f_height),
            bottom_left: Vector2::new(0.0, 0.0),
            bottom_right: Vector2::new(f_width, 0.0),
        }
    }

    pub fn collided_velocity(
        &self,
        position: &Vector2<f32>,
        distance: f32,
        direction: &Vector2<f32>,
    ) -> CollisionResult {
        let new_position = Vector2::new(
            position.x + (distance * direction.x),
            position.y + (distance * direction.y),
        );
        let top = self.top_collided(&position, &new_position, &direction);
        let bottom = self.bottom_collided(&position, &new_position, &direction);
        let left = self.left_collided(&position, &new_position, &direction);
        let right = self.right_collided(&position, &new_position, &direction);
        if top.is_some() {
            top.unwrap()
        } else if bottom.is_some() {
            bottom.unwrap()
        } else if left.is_some() {
            left.unwrap()
        } else if right.is_some() {
            right.unwrap()
        } else {
            CollisionResult {
                position: new_position,
                direction: direction.clone(),
            }
        }
    }

    fn top_collided(
        &self,
        current: &Vector2<f32>,
        projected: &Vector2<f32>,
        direction: &Vector2<f32>,
    ) -> Option<CollisionResult> {
        Self::line_collision(&self.top_left, &self.top_right, &current, &projected).map_or(
            None,
            |_collide_point| {
                let new_direction = Vector2::new(direction.x, -direction.y);
                Some(CollisionResult {
                    position: current.clone(), //collide_point.clone(),
                    direction: new_direction,
                })
            },
        )
    }

    fn bottom_collided(
        &self,
        current: &Vector2<f32>,
        projected: &Vector2<f32>,
        direction: &Vector2<f32>,
    ) -> Option<CollisionResult> {
        Self::line_collision(&self.bottom_left, &self.bottom_right, &current, &projected).map_or(
            None,
            |_collide_point| {
                let new_direction = Vector2::new(direction.x, -direction.y);
                Some(CollisionResult {
                    position: current.clone(), //new_position,
                    direction: new_direction,
                })
            },
        )
    }

    fn left_collided(
        &self,
        current: &Vector2<f32>,
        projected: &Vector2<f32>,
        direction: &Vector2<f32>,
    ) -> Option<CollisionResult> {
        Self::line_collision(&self.top_left, &self.bottom_left, &current, &projected).map_or(
            None,
            |_collide_point| {
                let new_direction = Vector2::new(-direction.x, direction.y);
                Some(CollisionResult {
                    position: current.clone(), //new_position,
                    direction: new_direction,
                })
            },
        )
    }

    fn right_collided(
        &self,
        current: &Vector2<f32>,
        projected: &Vector2<f32>,
        direction: &Vector2<f32>,
    ) -> Option<CollisionResult> {
        Self::line_collision(&self.top_right, &self.bottom_right, &current, &projected).map_or(
            None,
            |_collide_point| {
                let new_direction = Vector2::new(-direction.x, direction.y);
                Some(CollisionResult {
                    position: current.clone(),
                    direction: new_direction,
                })
            },
        )
    }

    fn line_collision(
        p1_1: &Vector2<f32>,
        p1_2: &Vector2<f32>,
        p2_3: &Vector2<f32>,
        p2_4: &Vector2<f32>,
    ) -> Option<Vector2<f32>> {
        let div0 = (p2_4.y - p2_3.y) * (p1_2.x - p1_1.x) - (p2_4.x - p2_3.x) * (p1_2.y - p1_1.y);
        let num1 = (p2_4.x - p2_3.x) * (p1_1.y - p2_3.y) - (p2_4.y - p2_3.y) * (p1_1.x - p2_3.x);
        let num2 = (p1_2.x - p1_1.x) * (p1_1.y - p2_3.y) - (p1_2.y - p1_1.y) * (p1_1.x - p2_3.x);

        if div0 == 0.0 {
            // warning: using equals on floats, tbd
            if num1 == 0.0 && num2 == 0.0 {
                Some(p2_3.clone())
            } else {
                None
            }
        } else {
            let r = num1 / div0;
            let s = num2 / div0;

            if (r >= 0.0 && r <= 1.0) && (s >= 0.0 && s <= 1.0) {
                Some(Vector2::new(
                    p1_1.x + (num1 * (p1_2.x - p1_1.x)),
                    p1_1.y + (num1 * (p1_2.y - p1_1.y)),
                ))
            } else {
                None
            }
        }
    }
}
