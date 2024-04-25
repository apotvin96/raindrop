extern crate nalgebra_glm as glm;

mod asset_info;
mod gpu_info;
mod mesh;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::spawn,
};

use asset_info::AssetInfo;

pub use gpu_info::{BufferGpuInfo, ImageGpuInfo};
pub use mesh::{Mesh, Vertex};

pub struct AssetManager {
    meshes: Arc<Mutex<HashMap<String, Arc<Mutex<Mesh>>>>>,
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            meshes: Arc::new(Mutex::new(HashMap::new())),
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

        let mesh = Mesh {
            asset_info,
            gpu_info: None,
            vertices: vec![],
            vertex_count: 0,
        };

        let mesh = Arc::new(Mutex::new(mesh));

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
}

impl Default for AssetManager {
    fn default() -> Self {
        AssetManager::new()
    }
}
