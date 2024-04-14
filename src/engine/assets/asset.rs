use std::{cell::RefCell, rc::Rc};

/// A Generic interface for an asset needed in the system, with a path to the asset.
///
/// In the case of a mesh, this would be the path to the mesh file, and once needed,
/// the mesh would be loaded into memory, stored in the asset manager, and then
/// uploaded to the GPU. Ideally after this point, the asset stored in ram would be
/// dropped, and the asset would be accessed through the GPU.
pub struct Asset<T> {
    /// The path to the asset, currently only for on disk
    path: String,

    /// Do we need to reload this asset from disk?
    needs_reload: bool,

    /// The asset itself, if it exists, if already uploaded to the GPU, this will be None
    asset: Option<Rc<RefCell<T>>>,

    /// Has this asset been uploaded to the GPU? (or equivalent, being parsed, optimized, etc.)
    uploaded: bool,
}

impl<T> Asset<T> {
    /// Create a new asset with a given path.
    ///
    /// This asset will need to be loaded at some point
    pub fn new(path: String) -> Self {
        Asset {
            path: path,
            needs_reload: true,
            uploaded: false,
            asset: None,
        }
    }

    /// Get a handle of the asset, if it exists.
    ///
    /// If it does, it will return a reference counted clone of the asset.
    pub fn get(&self) -> Option<Rc<RefCell<T>>> {
        if self.asset.is_some() {
            Some(self.asset.as_ref().unwrap().clone())
        } else {
            None
        }
    }
}
