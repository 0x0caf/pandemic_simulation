use rgx::core::*;
use rgx::kit::shape2d::{Batch, Shape, Fill};
use rgx::math::*;
use rand::prelude::*;
use crate::window_box::WindowBox;
use std::f32::consts::PI;


pub struct WindowAttributes {
    pub width: f32,
    pub height: f32,
}

struct CircleState {
    position: Vector2<f32>,
    velocity: f32,
    direction: Vector2<f32>,
}

pub struct SimulationApp {
    window: WindowAttributes,
    window_box: WindowBox,
    circle_state: CircleState,
    circles: Vec<CircleState>,
}

impl SimulationApp {
    pub fn new(window: WindowAttributes) -> SimulationApp {
        let position = Vector2::new( window.width * 0.5, window.height * 0.5);

        let num_circles = 500;
        let mut circles = Vec::new();
        for i in 1..num_circles {
            circles.push(
                Self::new_circle(window.width, window.height)
            )
        }

        SimulationApp {
            window_box: WindowBox::new(window.width, window.height),
            window,
            circles,
            circle_state: CircleState {
                position,
                velocity: 1000.0,
                direction: Vector2::new(0.3, 1.0).normalize()
            },
        }
    }

    pub fn update(&mut self, delta_time: i64) {
        let shift = self.circle_state.velocity * (delta_time as f32)/1000.0;
        let result = self.window_box.collided_velocity(&self.circle_state.position, shift, &self.circle_state.direction);
        self.circle_state.position = result.position.clone();
        self.circle_state.direction = result.direction.clone();

        for circle in self.circles.iter_mut() {
            let shift = circle.velocity * (delta_time as f32)/1000.0;
            let result = self.window_box.collided_velocity(&circle.position, shift, &circle.direction);
            circle.position = result.position;
            circle.direction = result.direction;
        }
    }

    pub fn render(&self) -> Batch {
        let mut batch = Batch::new();

        let circle_size = 3.0;
        let circle_color = Rgba::new(1.0, 0.0, 0.0, 1.0);

        for circle in self.circles.iter() {
            batch.add(
                Shape::circle(Point2::new(circle.position.x, circle.position.y), circle_size, 8)
                    .fill(Fill::Solid(circle_color.clone()))
                    .stroke(1.0, circle_color.clone()),
            )
        }
        batch
    }

    fn new_circle(width: f32, height: f32) -> CircleState {
        let mut rng = rand::thread_rng();
        let x = width * rng.gen::<f32>();
        let y = height * rng.gen::<f32>();
        let velocity = 100. * rng.gen::<f32>();
        let angle = 2. * PI * rng.gen::<f32>();
        let ang_x = angle.cos();
        let ang_y = angle.sin();

        CircleState {
            position: Vector2::new(x, y),
            velocity,
            direction: Vector2::new(ang_x, ang_y)
        }
    }
}