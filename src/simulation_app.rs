use crate::organism::{InfectionRadius, OrganismState};
use crate::window_box::WindowBox;
use rgx::kit::shape2d::Batch;
use rgx::math::*;
use crate::grid_system::GridSystem;

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
    grid_system: GridSystem,
    radius: f32,
    frame: u32,
}

impl SimulationApp {
    pub fn new(window: WindowAttributes) -> SimulationApp {
        let num_organisms = 20000;
        let mut num_infected = 0;
        let max_infected = 50;
        let percent_in_place = 0.;
        let circle_radius = 1.0;
        let infection_lifetime_ms = 1000;
        let fatality_rate = 10.0;
        // proportions
        let grid_pixel_size = 20;
        let max_velocity = 100.;

        let window_box = WindowBox::new(window.width, window.height, grid_pixel_size);
        let mut infection_group = Vec::new();
        let mut organisms = Vec::new();

        let grid_system = GridSystem::new(window.width, window.height, grid_pixel_size);

        for i in 1..num_organisms {
            let mut organism = OrganismState::random(
                window.width as f32,
                window.height as f32,
                circle_radius,
                max_velocity,
                percent_in_place,
                infection_lifetime_ms,
                fatality_rate,
                &window_box,
                &grid_system,
            );
            if num_infected < max_infected {
                organism.set_infected();
                infection_group.push(organism.get_infection_radius());
            }

            organisms.push(organism);
            num_infected = num_infected + 1;
        }

        SimulationApp {
            window_box,
            window,
            organisms,
            grid_system,
            radius: circle_radius,
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
                organism.check_infected(&self.infection_group, &mut infection_group, &mut self.grid_system);
            }
            self.infection_group = infection_group;
        }
        self.frame = self.frame + 1;
    }

    pub fn render(&self) -> Batch {
        let mut batch = Batch::new();
        for organism in self.organisms.iter() {
            organism.render(self.radius, &mut batch, self.frame);
        }
        batch
    }
}
