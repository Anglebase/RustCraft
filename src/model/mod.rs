use std::collections::{HashMap, HashSet};

pub mod model;
use model::Model;

pub struct ModelManager {
    models: HashMap<String, Model>,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, vertices: Vec<f32>, indices: Vec<u32>, description: &str) {
        self.models.insert(
            String::from(name),
            Model::new(vertices, indices, description),
        );
    }

    pub fn get(&self, name: &str) -> Option<&Model> {
        self.models.get(name)
    }
}

use lazy_static::lazy_static;

use crate::{warn, RustCraftWrapper};

lazy_static! {
    pub static ref MODEL_MANAGER: RustCraftWrapper<ModelManager> =
        RustCraftWrapper::new(ModelManager::new());
}

lazy_static! {
    static ref NOT_FOUND_MODEL: RustCraftWrapper<HashSet<String>> =
        RustCraftWrapper::new(HashSet::new());
}

impl RustCraftWrapper<ModelManager> {
    pub fn add_model(&self, name: &str, vertices: Vec<f32>, indices: Vec<u32>, description: &str) {
        self.apply(|manager| manager.add(name, vertices, indices, description));
    }

    pub fn draw_model(&self, name: &str) {
        self.apply(|manager| {
            if let Some(model) = manager.get(name) {
                model.draw();
            } else {
                NOT_FOUND_MODEL.apply(|set| {
                    if set.contains(name) {
                        return;
                    }
                    warn!("RCW<ModelManager>", "找不到模型 {}", name);
                    set.insert(String::from(name));
                });
            }
        });
    }
}
