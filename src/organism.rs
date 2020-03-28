use crate::window_box::WindowBox;
use rand::prelude::*;
use rgx::core::*;
use rgx::kit::shape2d::{Batch, Fill, Shape};
use rgx::math::*;
use std::f32::consts::PI;

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
    infection_lifetime_ms: i64,
    fatality_rate: f32,
    pub grid_id: i32,
    infection_state: InfectionState,
}

pub struct InfectionRadius {
    pub position: Vector2<f32>,
    pub grid_id: i32,
}

impl OrganismState {
    pub fn random(
        width: f32,
        height: f32,
        max_velocity: f32,
        percentage_in_place: f32,
        infection_lifetime_ms: i64,
        fatality_rate: f32,
        window_box: &WindowBox,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let x = width * rng.gen::<f32>();
        let y = height * rng.gen::<f32>();
        let velocity = if rng.gen::<f32>() * 100. < percentage_in_place {
            0.
        } else {
            max_velocity * rng.gen::<f32>()
        };
        let angle = 2. * PI * rng.gen::<f32>();
        let ang_x = angle.cos();
        let ang_y = angle.sin();
        let position = Vector2::new(x, y);

        Self {
            position,
            velocity,
            infection_time: 0,
            infection_lifetime_ms,
            fatality_rate,
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
            if self.infection_time >= self.infection_lifetime_ms {
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() * 100. < self.fatality_rate {
                    self.infection_state = InfectionState::Dead;
                    self.velocity = 0.0;
                } else {
                    self.infection_state = InfectionState::Recovered;
                }
                self.infection_time = 0;
            }
        }
    }

    pub fn check_infected(
        &mut self,
        collision_radius: f32,
        previous_infected_group: &Vec<InfectionRadius>,
        infected_group: &mut Vec<InfectionRadius>,
    ) {
        if self.infection_state == InfectionState::Uninfected {
            'inner: for radius in previous_infected_group.iter() {
                if self.grid_id == radius.grid_id {
                    let x_diff = self.position.x - radius.position.x;
                    let y_diff = self.position.y - radius.position.y;
                    let dot = x_diff * x_diff + y_diff * y_diff;
                    if dot <= collision_radius * collision_radius {
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

    pub fn render(&self, radius: f32, batch: &mut Batch, frame: u32) {
        let color = match self.infection_state {
            InfectionState::Uninfected => Rgba::new(0.0, 0.5, 0.0, 1.0),
            InfectionState::Infected => Rgba::new(1.0, 0.0, 0.0, 1.0),
            InfectionState::Recovered => Rgba::new(0.5, 0.5, 0.5, 1.0),
            InfectionState::Dead => {
                if frame >> 3 & 0x1 == 1 {
                    Rgba::new(1.0, 0., 1.0, 1.0)
                } else {
                    Rgba::new(0.0, 0., 0., 1.0)
                }
            }
        };
        batch.add(
            Shape::rect(
                Point2::new(
                    self.position.x + radius * 0.5,
                    self.position.y + radius * 0.5,
                ),
                Point2::new(
                    self.position.x - radius * 0.5,
                    self.position.y - radius * 0.5,
                ),
            )
            .fill(Fill::Solid(color.clone()))
            .stroke(1.0, color.clone()),
        );
    }
}
