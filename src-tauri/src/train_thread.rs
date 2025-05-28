use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::common::coords_lock::{CoordinatesLock};
use crate::common::line_accumulator::LineAccumulator;
use crate::common::path_iterator::PathIterator;
use crate::common::structs::{Coords};
use crate::config::{MetroConfigTrain};
use crate::ipc::{IpcActionRenderPoints, IpcActionRenderPointsPayloadPoint};

pub struct Train {
    train_config: MetroConfigTrain,
    stop_flag: Arc<AtomicBool>,
    coordinates_lock: Arc<CoordinatesLock>,
    on_update: Option<Arc<dyn Fn(IpcActionRenderPoints) + Send + Sync>>,
    cars_coords: Vec<Coords>,
}

impl Train {
    pub fn new(
        train_config: MetroConfigTrain,
        stop_flag: Arc<AtomicBool>,
        coordinates_lock: Arc<CoordinatesLock>,
        on_update: Option<Arc<dyn Fn(IpcActionRenderPoints) + Send + Sync>>,
    ) -> Self {
        Self {
            train_config,
            stop_flag,
            coordinates_lock,
            on_update,
            cars_coords: Vec::new(),
        }
    }

    pub fn start(self) {
        thread::spawn(move || self.run());
    }

    fn run(mut self) {
        let path_coords = Coords::from_pairs_vector(self.train_config.path.clone());
        let mut path_iterator = PathIterator::new(&path_coords);

        // lifetime - outside of loop
        let mut locks = Vec::new();
        let mut _guards = Vec::new();

        while let Some(path_element) = path_iterator.next() {
            if self.stop_flag.load(Ordering::Relaxed) {
                println!("Thread {} received stop signal, exiting.", self.train_config.id);
                break;
            }
            self.cars_coords.push(path_element.coords.clone());

            let mut coords_to_lock = Vec::new();
            coords_to_lock.extend(self.cars_coords.clone());
            let straight_elements =
                LineAccumulator::from_peek_fn(|i| path_iterator.peek(i).map(|el| el.coords))
                    .collect();
            coords_to_lock.extend(straight_elements.clone());


            let _main_guard = self.coordinates_lock.main_lock.lock();
            locks.clear();
            _guards.clear();
            self.coordinates_lock.lock_coords_list(coords_to_lock).iter().for_each(| l| {
                locks.push(l.clone());
                _guards.push(l.lock());
            });

  
            if self.cars_coords.len() > self.train_config.cars.len() {
                let removed = self.cars_coords.remove(0);
                if let Some(cb) = &self.on_update {
                    cb(IpcActionRenderPoints {
                        display_action_type: "clear_coords".to_string(),
                        display_action_payload: vec![IpcActionRenderPointsPayloadPoint {
                            coords: removed.to_pair(),
                            bg_color: "".to_string(),
                            fg_color: "".to_string(),
                        }],
                    });
                }
            }

            if let Some(cb) = &self.on_update {
                let mut payload = vec![];
                for (i, &coords) in self.cars_coords.iter().rev().enumerate() {
                    if i >= self.train_config.cars.len() { break; }
                    let car_config = &self.train_config.cars[i];
                    payload.push(IpcActionRenderPointsPayloadPoint {
                        coords: coords.to_pair(),
                        bg_color: car_config.bg_color.clone(),
                        fg_color: "white".to_string(),
                    });
                }
                cb(IpcActionRenderPoints {
                    display_action_type: "add_cars".to_string(),
                    display_action_payload: payload,
                });
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}