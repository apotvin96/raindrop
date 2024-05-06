extern crate nalgebra_glm as glm;

mod asset_info;
mod mesh;
mod sound;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::spawn,
};

use asset_info::AssetInfo;

pub use mesh::{Mesh, Vertex};
use sound::Sound;

pub struct AssetManager {
    meshes: Arc<Mutex<HashMap<String, Arc<Mutex<Mesh>>>>>,
    sounds: Arc<Mutex<HashMap<String, Arc<Mutex<Sound>>>>>,
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            meshes: Arc::new(Mutex::new(HashMap::new())),
            sounds: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn iter_meshes_mut(&mut self) -> Arc<Mutex<HashMap<String, Arc<Mutex<Mesh>>>>> {
        self.meshes.clone()
    }

    pub fn get_mesh(&mut self, name: &String) -> Arc<Mutex<Mesh>> {
        let existing = {
            let meshes_binding = self.meshes.lock().unwrap();

            meshes_binding.get(name).cloned()
        };

        match existing {
            Some(mesh) => mesh,
            None => self.insert_mesh(name),
        }
    }

    fn insert_mesh(&mut self, name: &str) -> Arc<Mutex<Mesh>> {
        let asset_info = AssetInfo {
            id: name.to_owned(),
            status: asset_info::AssetStatus::Unloaded,
        };

        let mesh_info = Mesh {
            asset_info,
            gpu_info: None,
            vertices: vec![],
            vertex_count: 0,
        };

        let mesh = Arc::new(Mutex::new(mesh_info));

        self.meshes
            .lock()
            .unwrap()
            .insert(name.to_owned(), mesh.clone());

        let closure_mesh = mesh.clone();
        spawn(move || {
            let mut mesh_binding = closure_mesh.lock().unwrap();

            mesh_binding.load();
        });

        mesh.clone()
    }

    pub fn get_audio(&mut self, name: &String) -> Arc<Mutex<Sound>> {
        let existing = {
            let sounds_binding = self.sounds.lock().unwrap();

            sounds_binding.get(name).cloned()
        };

        match existing {
            Some(sound) => sound,
            None => self.insert_audio(name),
        }
    }

    fn insert_audio(&mut self, name: &str) -> Arc<Mutex<Sound>> {
        let asset_info = AssetInfo {
            id: name.to_owned(),
            status: asset_info::AssetStatus::Unloaded,
        };

        let sound_info = Sound {
            asset_info,
            source: None,
        };

        let sound = Arc::new(Mutex::new(sound_info));

        self.sounds
            .lock()
            .unwrap()
            .insert(name.to_owned(), sound.clone());

        let closure_sound = sound.clone();
        spawn(move || {
            let mut sound_binding = closure_sound.lock().unwrap();

            sound_binding.load();
        });

        sound.clone()
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        AssetManager::new()
    }
}
