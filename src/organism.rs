use crate::area::{Area, AreaPtr};
use crate::grid_system::GridSystem;
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
    area: AreaPtr,
    pub velocity: f32,
    direction: Vector2<f32>,
    infection_time: i64,
    infection_lifetime_ms: i64,
    fatality_rate: f32,
    infection_state: InfectionState,
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
        grid_system: &GridSystem,
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
        let grid_id = grid_system.get_grid_index(&position);
        Self {
            position,
            area: Area::new(&position, grid_id, size),
            velocity,
            infection_time: 0,
            infection_lifetime_ms,
            fatality_rate,
            direction: Vector2::new(ang_x, ang_y),
            infection_state: InfectionState::Uninfected,
        }
    }

    pub fn set_infected(&mut self, grid_system: &mut GridSystem) {
        if self.infection_state == InfectionState::Uninfected {
            let grid_id = grid_system.get_grid_index(&self.position);
            grid_system.add_area(&self.area, grid_id);
            let mut area = (&*self.area).borrow_mut();
            area.grid_id = grid_id;
            self.infection_state = InfectionState::Infected;
        }
    }

    pub fn update(&mut self, delta_ms: i64, window_box: &WindowBox) {
        let shift = self.velocity * (delta_ms as f32) / 1000.0;
        let result = window_box.collided_velocity(&self.position, shift, &self.direction);
        self.position = result.position;
        self.direction = result.direction;
        (&*self.area).borrow_mut().square.update(&self.position);

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

    pub fn check_infected(&mut self, grid_system: &mut GridSystem) {
        let old_grid_id = (&*self.area).borrow().grid_id;
        let new_grid_id = grid_system.get_grid_index(&self.position);

        if self.infection_state == InfectionState::Uninfected {
            let grid_ids =
                grid_system.get_grid_id_list(&(&*self.area).borrow().square.add_half_size_bias());
            'outer: for grid_id in grid_ids.iter() {
                if grid_system.find_intersection_in_grid(*grid_id, &(&*self.area).borrow().square) {
                    self.infection_state = InfectionState::Infected;
                    break 'outer;
                }
            }
        }

        if self.infection_state == InfectionState::Infected {
            if old_grid_id != new_grid_id {
                grid_system.remove_area_from_grid((&*self.area).borrow().area_id, old_grid_id);
                grid_system.add_area(&self.area, new_grid_id);
                let mut area = (&*self.area).borrow_mut();
                area.grid_id = new_grid_id;
            }
        }
    }

    pub fn render(&self, batch: &mut Batch, frame: u32) {
        let color = match self.infection_state {
            InfectionState::Uninfected => Rgba::new(0.0, 0.5, 0.0, 1.0),
            InfectionState::Infected => Rgba::new(1.0, 0.0, 0.0, 1.0),
            InfectionState::Recovered => Rgba::new(0.25, 0.25, 0.25, 1.0),
            InfectionState::Dead => {
                if frame >> 3 & 0x1 == 1 {
                    Rgba::new(1.0, 0., 1.0, 1.0)
                } else {
                    Rgba::new(0.0, 0., 0., 1.0)
                }
            }
        };
        let square = &(&*self.area).borrow().square;
        batch.add(
            Shape::rect(
                Point2::new(square.bottom_left.x, square.bottom_left.y),
                Point2::new(square.top_right.x, square.top_right.y),
            )
            .fill(Fill::Solid(color.clone()))
            .stroke(1.0, color.clone()),
        );
    }
}
