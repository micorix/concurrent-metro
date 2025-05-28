use std::fs;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetroConfigPoint {
    pub(crate) coords: [u32; 2],
    pub(crate) bg_color: String,
    pub(crate) fg_color: String,
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetroConfigTrain {
    pub(crate) id: String,
    pub(crate) cars: Vec<MetroConfigCar>,
    pub(crate) path: Vec<[u32; 2]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetroConfigCar {
    pub(crate) bg_color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetroConfigGrid {
    pub(crate) size: u32,
    points: Vec<MetroConfigPoint>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetroConfig {
    pub(crate) grid: MetroConfigGrid,
    pub trains: Vec<MetroConfigTrain>,
}

impl MetroConfig {
    pub fn empty() -> Self {
        MetroConfig {
            grid: MetroConfigGrid {
                size: 0,
                points: Vec::new(),
            },
            trains: Vec::new(),
        }
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = fs::read_to_string(filename)?;
        let config: MetroConfig = serde_yaml::from_str(&config_data)?;
        config._validate()?;

        Ok(config)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    fn _validate(&self) -> Result<(), String> {
        if self.grid.size < 1 {
            return Err("Grid size has to be greater than zero".to_string());
        }
        if self.trains.is_empty() {
            return Err("At least one train must be defined".to_string());
        }
        
        let path_lengths: Vec<usize> = self.trains.iter().map(|t| t.path.len()).collect();

        if are_arr_elements_the_same(&path_lengths) {
            return Err("All trains must have the same number of cars".to_string());
        }
        
        for train in &self.trains {
            if train.cars.is_empty() {
                return Err(format!("Train {} has no cars", train.id));
            }
        }
        Ok(())
    }
}

fn are_arr_elements_the_same(arr: &[usize]) -> bool {
    if arr.is_empty() {
        return true;
    }
    let first = arr[0];
    arr.iter().all(|&item| item == first)
}
