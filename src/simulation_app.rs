use crate::grid_system::GridSystem;
use crate::organism::OrganismState;
use crate::window_box::WindowBox;
use rgx::kit::shape2d::Batch;

pub struct WindowAttributes {
    pub width: i32,
    pub height: i32,
}
// Contagious, Symptomatic, Recovered, Dead

pub struct SimulationApp {
    window_box: WindowBox,
    organisms: Vec<OrganismState>,
    grid_system: GridSystem,
    frame: u32,
}

impl SimulationApp {
    pub fn new(window: WindowAttributes) -> SimulationApp {
        let num_organisms = 6000;
        let mut num_infected = 0;
        let max_infected = 200;
        let percent_in_place = 50.;
        let circle_radius = 3.0;
        let infection_lifetime_ms = 1000;
        let fatality_rate = 2.0;
        // proportions
        let grid_pixel_size = 25;
        let max_velocity = 100.;

        let window_box = WindowBox::new(window.width, window.height);
        let mut organisms = Vec::new();
        let mut grid_system = GridSystem::new(window.width, window.height, grid_pixel_size);

        for _i in 1..num_organisms {
            let mut organism = OrganismState::random(
                window.width as f32,
                window.height as f32,
                circle_radius,
                max_velocity,
                percent_in_place,
                infection_lifetime_ms,
                fatality_rate,
                &grid_system,
            );
            if num_infected < max_infected && organism.velocity > 0. {
                organism.set_infected(&mut grid_system);
            }

            organisms.push(organism);
            num_infected = num_infected + 1;
        }

        SimulationApp {
            window_box,
            organisms,
            grid_system,
            frame: 0,
        }
    }

    pub fn update(&mut self, delta_time: i64) {
        // update all positions
        for organism in self.organisms.iter_mut() {
            organism.update(delta_time, &self.window_box);
        }
        // positions updated, now check for intersections
        for organism in self.organisms.iter_mut() {
            organism.check_infected(delta_time, &mut self.grid_system);
        }
        self.frame = self.frame + 1;
    }

    pub fn render(&self) -> Batch {
        let mut batch = Batch::new();
        for organism in self.organisms.iter() {
            organism.render(&mut batch, self.frame);
        }
        batch
    }
}
