
use rgx::math::*;
use rand::prelude::*;
use rgx::core::*;
use rgx::kit::shape2d::{Batch, Shape, Fill};
use std::f32::consts::PI;
use crate::window_box::WindowBox;

#[derive(PartialEq)]
enum InfectionState {
    Uninfected,
    Infected,
    Recovered,
    Dead,
}

pub struct OrganismState {
    pub position: Vector2<f32>,
    velocity: f32,
    direction: Vector2<f32>,
    infection_time: i64,
    pub grid_id: i32,
    infection_state: InfectionState,
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
            (width / 5.) * rng.gen::<f32>()
        };
        let angle = 2. * PI * rng.gen::<f32>();
        let ang_x = angle.cos();
        let ang_y = angle.sin();
        let position = Vector2::new(x, y);

        Self {
            position,
            velocity,
            infection_time: 0,
            direction: Vector2::new(ang_x, ang_y),
            grid_id: window_box.grid_id(&position),
            infection_state: InfectionState::Uninfected,
        }
    }

    pub fn set_infected(&mut self) {
        self.infection_state = InfectionState::Infected;
    }

    pub fn is_infected(&self) -> bool {
        self.infection_state == InfectionState::Infected
    }

    pub fn update(&mut self, delta_ms: i64, window_box: &WindowBox) {
        let shift = self.velocity * (delta_ms as f32) / 1000.0;
        let result = window_box.collided_velocity(&self.position, shift, &self.direction);
        self.position = result.position;
        self.direction = result.direction;
        self.grid_id = window_box.grid_id(&result.position);
        if self.infection_state == InfectionState::Infected {
            self.infection_time += delta_ms;
            if self.infection_time >= 5000 {
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() * 100. < 3.0 {
                    self.infection_state = InfectionState::Dead;
                    self.velocity = 0.0;
                } else {
                    self.infection_state = InfectionState::Recovered;
                }
                self.infection_time = 0;
            }
        }
    }

    pub fn check_infected(&mut self, collision_radius: f32, previous_infected_group: &Vec<InfectionRadius>, infected_group: &mut Vec<InfectionRadius>) {
        if self.infection_state == InfectionState::Uninfected {
            'inner: for radius in previous_infected_group.iter() {
                if self.grid_id == radius.grid_id {
                    if self.position.distance(radius.position) <= collision_radius {
                        self.infection_state = InfectionState::Infected;
                        break 'inner;
                    }
                }
            }
        }

        if self.infection_state == InfectionState::Infected {
            infected_group.push(InfectionRadius {
                position: self.position.clone(),
                grid_id: self.grid_id,
            });
        }
    }

    pub fn render(&self, radius: f32, batch: &mut Batch) {

        let color = match self.infection_state {
            InfectionState::Uninfected => Rgba::new(1.0, 1.0, 1.0, 1.0),
            InfectionState::Infected => Rgba::new(1.0, 0.0, 0.0, 1.0),
            InfectionState::Recovered => Rgba::new(0.0, 1.0, 0.0, 1.0),
            InfectionState::Dead => Rgba::new(0.75, 0.75, 0.75, 1.0),
        };
        batch.add(
            Shape::circle(Point2::new(self.position.x, self.position.y), radius, 4)
                .fill(Fill::Solid(color.clone()))
                .stroke(1.0, color.clone()),
        );
    }
}