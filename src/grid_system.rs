use crate::area::{AreaId, AreaPtr, GridId};
use crate::square::Square;
use rgx::math::*;
use std::rc::Rc;

#[allow(dead_code)]
struct Grid {
    radii: Vec<AreaPtr>,
    square: Square,
}

pub struct GridSystem {
    grids: Vec<Grid>,
    grid_size: f32,
    num_columns: i32,
}

impl GridSystem {
    pub fn new(width: i32, height: i32, grid_size: i32) -> GridSystem {
        // adding one to cover 0 to n, rather than 0 to n - 1
        let adjusted_width = ((width / grid_size) + 1) as i32;
        let adjusted_height = ((height / grid_size) + 1) as i32;
        let mut grids = vec![];
        let half_grid_size = grid_size >> 1;

        for y in 0..=adjusted_height {
            for x in 0..=adjusted_width {
                grids.push(Grid {
                    radii: vec![],
                    square: Square::new(
                        Vector2::new(
                            ((x * grid_size) + (x * half_grid_size)) as f32,
                            ((y * grid_size) + (y * half_grid_size)) as f32,
                        ),
                        grid_size as f32,
                    ),
                });
            }
        }

        GridSystem {
            grids,
            num_columns: adjusted_width,
            grid_size: grid_size as f32,
        }
    }

    pub fn get_grid_index(&self, position: &Vector2<f32>) -> usize {
        let x = (position.x as i32) / (self.grid_size as i32);
        let y = (position.y as i32) / (self.grid_size as i32);
        (x + (y * self.num_columns)) as GridId
    }

    pub fn get_grid_id_list(&self, square: &Square) -> Vec<GridId> {
        let mut ids = vec![
            self.get_grid_index(&square.bottom_left),
            self.get_grid_index(&Vector2::new(square.top_right.x, square.bottom_left.y)),
            self.get_grid_index(&square.center),
            self.get_grid_index(&Vector2::new(square.bottom_left.x, square.top_right.y)),
            self.get_grid_index(&square.top_right),
        ];
        ids.sort();
        ids.dedup();
        ids
    }

    pub fn add_area(&mut self, area: &AreaPtr, grid_id: GridId) -> GridId {
        if let Some(grid) = self.grids.get_mut(grid_id) {
            grid.radii.push(Rc::clone(area));
        }
        grid_id
    }

    pub fn remove_area_from_grid(&mut self, area_id: AreaId, grid_id: GridId) {
        if let Some(grid) = self.grids.get_mut(grid_id) {
            for index in 0..grid.radii.len() {
                if let Some(area) = grid.radii.get(index) {
                    if (&*area).borrow().area_id == area_id {
                        grid.radii.remove(index);
                        return;
                    }
                }
            }
        }
    }

    pub fn find_intersection_in_grid(&self, grid_id: GridId, square: &Square) -> bool {
        if let Some(grid) = self.grids.get(grid_id) {
            for area in grid.radii.iter() {
                if (&*area).borrow().square.intersects(square) {
                    return true;
                }
            }
            return false;
        }
        false
    }
}
