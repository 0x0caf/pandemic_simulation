use crate::window_box::WindowBox;
use rgx::math::*;
use rand::prelude::*;
use rgx::kit::shape2d::Batch;
use std::f32::consts::PI;

pub struct OrganismState {
    pub position: Vector2<f32>,
    velocity: f32,
    direction: Vector2<f32>,
    pub grid_id: i32,
    pub infected: bool,
}

pub struct InfectionRadius {
    pub position: Vector2<f32>,
    pub grid_id: i32,
}

impl OrganismState {
    pub fn random(width: f32, height: f32, percentage_in_place: f32, window_box: &WindowBox) -> Self {
        let mut rng = rand::thread_rng();
        let x = width * rng.gen::<f32>();
        let y = height * rng.gen::<f32>();
        let velocity = if rng.gen::<f32>() * 100. < percentage_in_place {
            0.
        } else {
            100. * rng.gen::<f32>()
        };
        let angle = 2. * PI * rng.gen::<f32>();
        let ang_x = angle.cos();
        let ang_y = angle.sin();
        let position = Vector2::new(x, y);

        Self {
            position,
            velocity,
            direction: Vector2::new(ang_x, ang_y),
            grid_id: window_box.grid_id(&position),
            infected: false,
        }
    }

    pub fn update(&mut self, delta_ms: i64, window_box: &WindowBox) {
        let shift = self.velocity * (delta_ms as f32) / 1000.0;
        let result = window_box.collided_velocity(&self.position, shift, &self.direction);
        self.position = result.position;
        self.direction = result.direction;
        self.grid_id = window_box.grid_id(&result.position);
    }

    pub fn check_infected(&mut self, collision_radius: f32, previous_infected_group: &Vec<InfectionRadius>, infected_group: &mut Vec<InfectionRadius>) {
        if !self.infected {
            'inner: for radius in previous_infected_group.iter() {
                if self.grid_id == radius.grid_id {
                    self.infected = self.position.distance(radius.position) <= collision_radius;
                    if self.infected {
                        break 'inner;
                    }
                }
            }
        }

        if self.infected {
            infected_group.push( InfectionRadius {
                position: self.position.clone(),
                grid_id: self.grid_id,
            });
        }
    }

    pub fn render(&self, batch: &mut Batch) {

    }
}