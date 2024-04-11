use std::{cell::RefCell, rc::Rc};

pub struct Asset<T> {
    path: String,
    asset: Rc<RefCell<T>>,
}

impl<T> Asset<T> {
    pub fn new(path: String, asset: T) -> Self {
        Asset {
            path: path,
            asset: Rc::new(RefCell::new(asset)),
        }
    }

    pub fn get(&self) -> Rc<RefCell<T>> {
        self.asset.clone()
    }
}
