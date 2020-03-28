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

#[derive(Clone)]
pub struct Square {
    bottom_left: Vector2<f32>,
    top_right: Vector2<f32>,
}

impl Square {
    fn from_center(position: &Vector2<f32>, size: f32) -> Square {
        let half_size = size * 0.5;
        let left = position.x - half_size;
        let right = position.x + half_size;
        let bottom = position.y - half_size;
        let top = position.y + half_size;
        Square {
            bottom_left: Vector2::new(left, bottom),
            top_right: Vector2::new(right, top),
        }
    }

    fn intersects(&self, test: &Square) -> bool {
        let y_intersect = (test.bottom_left.y >= self.bottom_left.y
            && test.bottom_left.y <= self.top_right.y)
            || (test.top_right.y >= self.bottom_left.y && test.top_right.y <= self.top_right.y);

        let x_intersect = (test.bottom_left.x >= self.bottom_left.x
            && test.bottom_left.x <= self.top_right.x)
            || (test.top_right.x >= self.bottom_left.x && test.top_right.x <= self.top_right.x);

        y_intersect && x_intersect
    }
}

pub struct OrganismState {
    pub position: Vector2<f32>,
    square: Square,
    size: f32,
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
    pub square: Square,
    pub grid_id: i32,
}

impl OrganismState {
    pub fn random(
        width: f32,
        height: f32,
        size: f32,
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
            square: Square::from_center(&position, size),
            size,
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

    pub fn get_infection_radius(&self) -> InfectionRadius {
        InfectionRadius {
            position: self.position.clone(),
            square: self.square.clone(),
            grid_id: self.grid_id,
        }
    }

    pub fn update(&mut self, delta_ms: i64, window_box: &WindowBox) {
        let shift = self.velocity * (delta_ms as f32) / 1000.0;
        let result = window_box.collided_velocity(&self.position, shift, &self.direction);
        self.position = result.position;
        self.square = Square::from_center(&self.position, self.size);
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
                    if self.square.intersects(&radius.square) {
                        self.infection_state = InfectionState::Infected;
                        break 'inner;
                    }
                }
            }
        }

        if self.infection_state == InfectionState::Infected {
            infected_group.push(self.get_infection_radius());
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
                Point2::new(self.square.bottom_left.x, self.square.bottom_left.y),
                Point2::new(self.square.top_right.x, self.square.top_right.y),
            )
            .fill(Fill::Solid(color.clone()))
            .stroke(1.0, color.clone()),
        );
    }
}
