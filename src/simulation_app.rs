use rgx::core::*;
use rgx::kit::shape2d::{Batch, Shape, Fill};
use rgx::math::*;
use rand::prelude::*;
use crate::window_box::WindowBox;
use std::f32::consts::PI;
use crate::organism::{OrganismState, InfectionRadius};


pub struct WindowAttributes {
    pub width: i32,
    pub height: i32,
}

struct CircleState {
    position: Vector2<f32>,
    velocity: f32,
    direction: Vector2<f32>,
    grid_id: i32,
    infected: bool,
}
// Contagious, Symptomatic, Recovered, Dead

pub struct SimulationApp {
    window: WindowAttributes,
    window_box: WindowBox,
    organisms: Vec<OrganismState>,
    infection_group: Vec<InfectionRadius>,
    radius: f32,
    collision_radius: f32,
    frame: u32,
}

impl SimulationApp {
    pub fn new(window: WindowAttributes) -> SimulationApp {
        let num_organisms = 5000;
        let mut num_infected = 0;
        let max_infected = 1;
        let percent_in_place = 99.;
        let grid_pixel_size = 10;
        let circle_radius = 3.0;
        let circle_collision_radius = circle_radius * 2.0;

        let window_box = WindowBox::new(window.width, window.height, grid_pixel_size);
        let mut infection_group = Vec::new();
        let mut organisms = Vec::new();

        for i in 1..num_organisms {
            let mut organism = OrganismState::random(window.width as f32, window.height as f32, percent_in_place, &window_box);
            if num_infected < max_infected {
                organism.set_infected();
            }

            if organism.is_infected() {
                infection_group.push(
                    InfectionRadius {
                        position: organism.position.clone(),
                        grid_id: organism.grid_id
                    }
                )
            }
            organisms.push(
                organism
            );
            num_infected = num_infected + 1;
        }

        SimulationApp {
            window_box,
            window,
            organisms,
            radius: circle_radius,
            collision_radius: circle_collision_radius,
            infection_group,
            frame: 0,
        }
    }

    pub fn update(&mut self, delta_time: i64) {

        // update all positions
        for organism in self.organisms.iter_mut() {
            organism.update(delta_time, &self.window_box);
        }

        // all infected?
        if self.organisms.len() != self.infection_group.len() {
            // iterate through the circle group and check for newly infected
            let mut infection_group = Vec::new();
            for organism in self.organisms.iter_mut() {
                organism.check_infected(self.collision_radius, &self.infection_group, &mut infection_group);
            }
            self.infection_group = infection_group;
        }

        self.frame = self.frame + 1;
    }

    pub fn render(&self) -> Batch {
        let mut batch = Batch::new();
        for organism in self.organisms.iter() {
            organism.render(self.radius, &mut batch);
        }
        batch
    }
}