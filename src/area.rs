use rgx::math::*;
use rand::prelude::*;
use crate::square::Square;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub type GridId = usize;
pub type AreaId = i64;
pub type AreaPtr = Rc<RefCell<Area>>;

pub struct Area {
    pub area_id: AreaId, // unique id of this area(not to be confused with grid id)
    pub square: Square,
    pub grid_id: GridId,
}

impl Area {
    pub fn new(center: &Vector2<f32>, grid_id: GridId, size: f32) -> AreaPtr {
        let mut rng = rand::thread_rng();
        let area = Area {
            area_id: rng.gen::<AreaId>(),
            grid_id,
            square: Square::new(center.clone(), size),
        };
        Rc::new(RefCell::new(area))
    }

    fn update(&mut self, center: &Vector2<f32>, grid_id: GridId) {
        self.square.update(center);
        self.grid_id = grid_id;
    }
}

impl Ord for Area {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.area_id < other.area_id {
            Ordering::Less
        } else if self.area_id > other.area_id {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        self.area_id == other.area_id
    }
}

impl Eq for Area {}
