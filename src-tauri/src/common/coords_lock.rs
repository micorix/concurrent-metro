use parking_lot::{Mutex};
use std::collections::HashMap;
use std::sync::Arc;
use crate::common::structs::Coords;

#[derive(Clone)]
pub struct CoordinatesLock {
    coords_locks: Arc<Mutex<HashMap<Coords, Arc<Mutex<()>>>>>,
    pub main_lock: Arc<Mutex<()>>
}

impl CoordinatesLock {
    pub fn new() -> Self {
        Self {
            coords_locks: Arc::new(Mutex::new(HashMap::new())),
            main_lock: Arc::new(Mutex::new(()))
        }
    }

    fn get_coords_lock(&self, coords: &Coords) -> Arc<Mutex<()>> {
        let mut locks_map = self.coords_locks.lock();
        locks_map
            .entry(coords.clone())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }

    pub fn lock_coords_list(&self, coords_list: Vec<Coords>) -> Vec<Arc<Mutex<()>>> {
        // Make locking deterministic using sort
        let mut sorted_coords = coords_list.clone();
        sorted_coords.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
        sorted_coords.dedup();

        let _ = self.main_lock.lock();
        let locks: Vec<_> = sorted_coords.iter()
            .map(|coords| self.get_coords_lock(coords))
            .collect();

        locks
    }

    pub fn get_locked_coords(&self) -> Vec<Coords> {
        let locks_map = self.coords_locks.lock();
        locks_map
            .iter()
            .filter_map(|(coords, lock)| if lock.try_lock().is_none() { Some(coords.clone()) } else { None })
            .collect()
    }
}
